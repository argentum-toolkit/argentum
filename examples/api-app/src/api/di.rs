use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_notification_business::mock::StdoutNotificator;
use argentum_oidc_infrastructure::default_di::OidcInfrastructureDiCBuilder;
use argentum_oidc_rest::ApiDiC as OidcApiDiC;
use argentum_rest_infrastructure::service::{BearerAuthenticator, RouterCombinator, Server};
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use std::env;
use std::net::SocketAddr;

use argentum_rest_infrastructure::RestDiC;
use argentum_user_account_infrastructure::di::UserAccountInfrastructureDiCBuilder;
use argentum_user_account_rest::ApiDiC as UserAccountApiDiC;
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

    const OIDC_CONNECTION_URL_ENV_NAME: &str = "AG_OIDC_DATABASE_URL";

    let oidc_database_url = env::var(OIDC_CONNECTION_URL_ENV_NAME)
        .map_err(|e| format!("Cant get ENV {OIDC_CONNECTION_URL_ENV_NAME}. Error: {e}"))?;

    const U_CONNECTION_URL_ENV_NAME: &str = "AG_USER_DATABASE_URL";

    let u_database_url = env::var(U_CONNECTION_URL_ENV_NAME)
        .map_err(|e| format!("Cant get ENV {U_CONNECTION_URL_ENV_NAME}. Error: {e}"))?;

    let unique_id_factory = Arc::new(UniqueIdFactory::new());

    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let client = reqwest::Client::builder()
        .user_agent("ArgentumToolKit/0.4")
        .pool_max_idle_per_host(20)
        .build()
        .map_err(|e| format!("Cant create reqwest client. Error: {e}"))?;
    let r_client = Arc::new(client);

    let rest_di = RestDiC::new(logger.clone());

    let u_di = Arc::new(
        UserInfrastructureDiCBuilder::new(unique_id_factory.clone(), logger.clone())
            //todo: use params
            .default_services(&u_database_url, 10, None, None)
            .await?
            .build()?,
    );

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

    // let pbkdf2_password_encryptor = Arc::new(Pbkdf2::new());

    let notificator = Arc::new(StdoutNotificator::new());

    const UA_CONNECTION_URL_ENV_NAME: &str = "AG_USER_ACCOUNT_DATABASE_URL";

    let database_url = env::var(UA_CONNECTION_URL_ENV_NAME)
        .map_err(|e| format!("Cant get ENV {UA_CONNECTION_URL_ENV_NAME}. Error: {e}"))?;

    const UA_MEMBER_JWT_SECRET: &str = "AG_USER_ACCOUNT_MEMBER_JWT_SECRET";

    let ua_member_token_secret = env::var(UA_MEMBER_JWT_SECRET)
        .map_err(|e| format!("Cant get ENV {UA_MEMBER_JWT_SECRET}. Error: {e}"))?;

    let ua_di = UserAccountInfrastructureDiCBuilder::new(
        u_di.clone(),
        unique_id_factory.clone(),
        logger.clone(),
        notificator,
    )
    .services(
        &database_url,
        10,
        &ua_member_token_secret,
        10 * 3600,
        None,
        None,
        None,
        logger.clone(),
    )
    .await?
    .config(
        "Argentum ToolKit. Example of API application".to_string(),
        3600, // TTL 1h
        "http://localhost:8082/change-password/".to_string(),
    )
    .build()?;

    let ua_api_di = UserAccountApiDiC::new(
        "/api/v1".to_string(),
        rest_di.request_transformer.clone(),
        bearer_authenticator.clone(),
        ua_di.anonymous_registers_handler,
        ua_di.anonymous_requests_restore_token_handler,
        ua_di.anonymous_with_token_changes_password_handler,
        ua_di.member_verifies_email_handler,
        ua_di.user_logins_with_password_handler,
        ua_di.user_registers_with_password_handler,
        rest_di.error_pre_handler.clone(),
    );

    const GITHUB_CLIENT_ID: &str = "GITHUB_CLIENT_ID";

    let github_client_id = env::var(GITHUB_CLIENT_ID)
        .map_err(|e| format!("Cant get ENV {GITHUB_CLIENT_ID}. Error: {e}"))?;

    const GITHUB_CLIENT_SECRET: &str = "GITHUB_CLIENT_SECRET";

    let github_client_secret = env::var(GITHUB_CLIENT_SECRET)
        .map_err(|e| format!("Cant get ENV {GITHUB_CLIENT_SECRET}. Error: {e}"))?;

    let o_di = OidcInfrastructureDiCBuilder::new(
        u_di.business_dic.session_repository.clone(),
        ua_di.business_dic.member_token_generator.clone(),
        u_di.business_dic.authenticated_user_repository.clone(),
        u_di.business_dic.guest_binder.clone(),
        unique_id_factory.clone(),
        logger.clone(),
        r_client.clone(), //TODO: don't duplicate client
    )
    .build(
        &oidc_database_url,
        10,
        &github_client_id,
        &github_client_secret,
        logger.clone(),
    )
    .await?;

    let o_api_di = OidcApiDiC::new(
        "/api/v1".to_string(),
        rest_di.request_transformer,
        bearer_authenticator,
        o_di.exchange_code_handler.clone(),
        rest_di.error_pre_handler.clone(),
    );

    // let listen = "172.18.0.1:8088";
    // let listen = "127.0.0.1:8088";
    let listen = "0.0.0.0:8088";
    let addr: SocketAddr = listen
        .parse()
        .map_err(|e| format!("Unable to parse socket address. Error: {e}"))?;

    let router = Arc::new(RouterCombinator::new(
        vec![u_api_di.router, ua_api_di.router, o_api_di.router],
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
