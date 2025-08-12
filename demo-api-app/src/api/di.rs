use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_notification_business::mock::StdoutNotificator;
use argentum_rest_infrastructure::service::{BearerAuthenticator, RouterCombinator, Server};
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use std::env;
use std::net::SocketAddr;
use std::rc::Rc;

use argentum_rest_infrastructure::RestDiC;
use argentum_user_account_infrastructure::di::UserAccountInfrastructureDiCBuilder;
use argentum_user_account_rest::ApiDiC;
use argentum_user_infrastructure::di::UserInfrastructureDiCBuilder;
use argentum_user_rest::ApiDiC as UserApiDiC;
use dotenvy::dotenv;
use std::sync::Arc;

pub struct DiC<L>
where
    L: LoggerTrait,
{
    // Public services
    pub server: Arc<Server<L>>,
}

impl<L> DiC<L>
where
    L: LoggerTrait,
{
    pub fn new(server: Arc<Server<L>>) -> Self {
        Self { server }
    }
}

pub async fn di_factory() -> Result<DiC<DefaultLogger<PrettyWriter>>, String> {
    dotenv().ok();

    const U_CONNECTION_URL_ENV_NAME: &str = "AG_USER_DATABASE_URL";

    let u_database_url = env::var(U_CONNECTION_URL_ENV_NAME)
        .map_err(|e| format!("Cant get ENV {U_CONNECTION_URL_ENV_NAME}. Error: {e}"))?;

    let unique_id_factory = Arc::new(UniqueIdFactory::new());

    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let u_di = Rc::new(
        UserInfrastructureDiCBuilder::new(unique_id_factory.clone())
            //todo: use params
            .default_services(&u_database_url, 5, logger.clone())
            .await?
            .build()?,
    );

    let rest_di = RestDiC::new(logger.clone());

    let bearer_authenticator = Arc::new(BearerAuthenticator::new(
        u_di.business_dic.user_authenticates_with_token_uc.clone(),
    ));

    let u_api_di = UserApiDiC::new(
        "/api/v1".to_string(),
        rest_di.request_transformer.clone(),
        bearer_authenticator.clone(),
        u_di.get_user_handler.clone(),
        rest_di.error_pre_handler.clone(),
    );

    let pbkdf2_password_encryptor = Arc::new(Pbkdf2::new());

    let notificator = Arc::new(StdoutNotificator::new());

    const UA_CONNECTION_URL_ENV_NAME: &str = "AG_USER_ACCOUNT_DATABASE_URL";

    let database_url = env::var(UA_CONNECTION_URL_ENV_NAME)
        .map_err(|e| format!("Cant get ENV {UA_CONNECTION_URL_ENV_NAME}. Error: {e}"))?;

    let ua_di = UserAccountInfrastructureDiCBuilder::new(
        u_di.clone(),
        unique_id_factory.clone(),
        pbkdf2_password_encryptor.clone(),
        pbkdf2_password_encryptor,
        logger.clone(),
        notificator,
    )
    .services(unique_id_factory, &database_url, 5, logger.clone())
    .await?
    .config(
        "Argentum ToolKit demo web application".to_string(),
        3600, // TTL 1h
        "http://localhost:8082/change-password/".to_string(),
    )
    .build()?;

    let ua_api_di = ApiDiC::new(
        "/api/v1".to_string(),
        rest_di.request_transformer,
        bearer_authenticator,
        ua_di.anonymous_registers_handler,
        ua_di.anonymous_requests_restore_token_handler,
        ua_di.anonymous_with_token_changes_password_handler,
        ua_di.user_logins_with_password_handler,
        ua_di.user_registers_with_password_handler,
        rest_di.error_pre_handler.clone(),
    );

    // let listen = "172.18.0.1:8088";
    // let listen = "127.0.0.1:8088";
    let listen = "0.0.0.0:8088";
    let addr: SocketAddr = listen
        .parse()
        .map_err(|e| format!("Unable to parse socket address. Error: {e}"))?;

    let router = Arc::new(RouterCombinator::new(
        vec![u_api_di.router, ua_api_di.router],
        rest_di.error_pre_handler,
    ));

    let server = Arc::new(Server::new(
        addr,
        router,
        rest_di.response_transformer,
        rest_di.error_handler,
        logger,
    ));

    Ok(DiC::new(server))
}
