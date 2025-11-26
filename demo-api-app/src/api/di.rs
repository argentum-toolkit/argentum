use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_notification_business::mock::StdoutNotificator;
use argentum_oidc_infrastructure::di::KeycloakInfrastructureDiCBuilder;
use argentum_oidc_rest::ApiDiC as KeycloakApiDiC;
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

    const U_CONNECTION_URL_ENV_NAME: &str = "AG_USER_DATABASE_URL";

    let u_database_url = env::var(U_CONNECTION_URL_ENV_NAME)
        .map_err(|e| format!("Cant get ENV {U_CONNECTION_URL_ENV_NAME}. Error: {e}"))?;

    let unique_id_factory = Arc::new(UniqueIdFactory::new());

    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let rest_di = RestDiC::new(logger.clone());

    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(20)
        .build()
        .map_err(|e| format!("Cant create reqwest client. Error: {e}"))?;
    let r_client = Arc::new(client);

    const ENV_KEYCLOAK_BASE_URL: &str = "KEYCLOAK_BASE_URL";
    let keycloak_base_url = env::var(ENV_KEYCLOAK_BASE_URL)
        .map_err(|e| format!("Cant get ENV {ENV_KEYCLOAK_BASE_URL}. Error: {e}"))?;

    const ENV_KEYCLOAK_REALM: &str = "KEYCLOAK_REALM";
    let keycloak_realm = env::var(ENV_KEYCLOAK_REALM)
        .map_err(|e| format!("Cant get ENV {ENV_KEYCLOAK_REALM}. Error: {e}"))?;

    const ENV_KEYCLOAK_FE_CLIENT_ID: &str = "KEYCLOAK_FE_CLIENT_ID";
    let keycloak_fe_client_id = env::var(ENV_KEYCLOAK_FE_CLIENT_ID)
        .map_err(|e| format!("Cant get ENV {ENV_KEYCLOAK_FE_CLIENT_ID}. Error: {e}"))?;

    const ENV_KEYCLOAK_FE_CLIENT_SECRET: &str = "KEYCLOAK_FE_CLIENT_SECRET";
    let keycloak_fe_client_secret = env::var(ENV_KEYCLOAK_FE_CLIENT_SECRET)
        .map_err(|e| format!("Cant get ENV {ENV_KEYCLOAK_FE_CLIENT_SECRET}. Error: {e}"))?;

    const ENV_OIDC_REDIRECT_URI: &str = "OIDC_REDIRECT_URI";
    let oidc_redirect_uri = env::var(ENV_OIDC_REDIRECT_URI)
        .map_err(|e| format!("Cant get ENV {ENV_OIDC_REDIRECT_URI}. Error: {e}"))?;

    const ENV_KEYCLOAK_BE_CLIENT_ID: &str = "KEYCLOAK_BE_CLIENT_ID";
    let keycloak_be_client_id = env::var(ENV_KEYCLOAK_BE_CLIENT_ID)
        .map_err(|e| format!("Cant get ENV {ENV_KEYCLOAK_BE_CLIENT_ID}. Error: {e}"))?;

    const ENV_KEYCLOAK_BE_CLIENT_SECRET: &str = "KEYCLOAK_BE_CLIENT_SECRET";
    let keycloak_be_client_secret = env::var(ENV_KEYCLOAK_BE_CLIENT_SECRET)
        .map_err(|e| format!("Cant get ENV {ENV_KEYCLOAK_BE_CLIENT_SECRET}. Error: {e}"))?;

    let k_di = KeycloakInfrastructureDiCBuilder::new(
        &keycloak_base_url,
        &keycloak_realm,
        &keycloak_fe_client_id,
        &keycloak_fe_client_secret,
        &oidc_redirect_uri,
        &keycloak_be_client_id,
        &keycloak_be_client_secret,
        logger.clone(),
        r_client.clone(),
    )
    .build()?;
    let k_api_di = KeycloakApiDiC::new(
        "/api/v1".to_string(),
        rest_di.request_transformer.clone(),
        k_di.exchange_code_handler.clone(),
        rest_di.error_pre_handler.clone(),
    );

    let u_di = Arc::new(
        UserInfrastructureDiCBuilder::new(unique_id_factory.clone())
            //todo: use params
            .default_services(
                &u_database_url,
                10,
                logger.clone(),
                k_di.member_repository.clone(),
                k_di.member_authenticator.clone(),
                k_di.token_router.clone(),
            )
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
    .services(
        &database_url,
        10,
        k_di.password_credential_repository,
        logger.clone(),
    )
    .await?
    .config(
        "Argentum ToolKit demo web application".to_string(),
        3600, // TTL 1h
        "http://localhost:8082/change-password/".to_string(),
    )
    .build()?;

    let ua_api_di = UserAccountApiDiC::new(
        "/api/v1".to_string(),
        rest_di.request_transformer,
        bearer_authenticator,
        ua_di.anonymous_registers_handler,
        ua_di.anonymous_requests_restore_token_handler,
        ua_di.anonymous_with_token_changes_password_handler,
        ua_di.member_verifies_email_handler,
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
        vec![u_api_di.router, ua_api_di.router, k_api_di.router],
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
