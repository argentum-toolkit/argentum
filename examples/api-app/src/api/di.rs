use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_notification_business::mock::StdoutNotifier;
use argentum_oidc_infrastructure::default_di::OidcInfrastructureDiBuilder;
use argentum_rest_infrastructure::service::{RouterCombinator, Server};
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_business::di::UserBusinessDiBuilder;
use argentum_user_infrastructure::di::UserInfrastructureDiBuilder;
use std::net::SocketAddr;

use argentum_rest_infrastructure::di::RestDiBuilder;
use argentum_user_account_infrastructure::di::UserAccountInfrastructureDiBuilder;
use std::sync::{Arc, Mutex};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Env {
    pub github_client_id: String,
    pub github_client_secret: String,
    pub ag_oidc_database_url: String,
    pub ag_user_database_url: String,
    pub ag_user_account_database_url: String,
    pub ag_user_account_member_jwt_secret: String,
}

pub struct Di<L>
where
    L: LoggerTrait,
{
    // Public services
    pub server: Arc<Server<L>>,
}

impl<L> Di<L>
where
    L: LoggerTrait,
{
    pub fn new(server: Arc<Server<L>>) -> Self {
        Self { server }
    }
}

pub async fn di_factory() -> Result<Di<DefaultLogger<PrettyWriter>>, String> {
    let env: Env = envy::from_env::<Env>().map_err(|e| format!("Cant load env. Error: {e}"))?;

    let unique_id_factory = Arc::new(UniqueIdFactory::new());

    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let client = reqwest::Client::builder()
        .user_agent("ArgentumToolKit/0.4")
        .pool_max_idle_per_host(20)
        .build()
        .map_err(|e| format!("Cant create reqwest client. Error: {e}"))?;
    let r_client = Arc::new(client);

    let user_business_builder = Arc::new(Mutex::new(UserBusinessDiBuilder::new(logger.clone())));
    let rest_di_builder = Arc::new(RestDiBuilder::new(
        user_business_builder.clone(),
        logger.clone(),
    ));

    let mut tmp_user_di_builder = UserInfrastructureDiBuilder::new(
        &env.ag_user_database_url,
        10,
        user_business_builder.clone(),
        rest_di_builder.clone(),
        logger.clone(),
    );
    let user_infrastructure_builder = Arc::new(Mutex::new(
        tmp_user_di_builder.with_default_services().await?,
    ));

    let notifier = Arc::new(StdoutNotifier::new());

    let pbkdf2 = Arc::new(Pbkdf2::new());

    let mut tmp_user_account_builder = UserAccountInfrastructureDiBuilder::new(
        "Argentum ToolKit. Example of API application",
        3600, // TTL 1h
        "http://localhost:8082/change-password/",
        &env.ag_user_account_database_url,
        10,
        &env.ag_user_account_member_jwt_secret,
        10 * 3600,
        user_business_builder.clone(),
        rest_di_builder.clone(),
        unique_id_factory.clone(),
        logger.clone(),
        notifier,
        pbkdf2.clone(),
        pbkdf2.clone(),
    );
    let user_account_infrastructure_builder =
        Arc::new(tmp_user_account_builder.wit_default_services().await?);

    let mut tmp_oidc_infrastructure_builder = OidcInfrastructureDiBuilder::new(
        &env.ag_oidc_database_url,
        10,
        &env.github_client_id,
        &env.github_client_secret,
        user_business_builder.clone(),
        user_account_infrastructure_builder.business_builder.clone(),
        rest_di_builder.clone(),
        unique_id_factory.clone(),
        logger.clone(),
        r_client.clone(), //TODO: don't duplicate client
    );

    let oidc_infrastructure_builder = tmp_oidc_infrastructure_builder
        .with_default_services()
        .await?;

    // let listen = "172.18.0.1:8088";
    // let listen = "127.0.0.1:8088";
    let listen = "0.0.0.0:8088";
    let addr: SocketAddr = listen
        .parse()
        .map_err(|e| format!("Unable to parse socket address. Error: {e}"))?;

    let rest_di = rest_di_builder.build();
    let user_infrastructure_di = Arc::new(user_infrastructure_builder.lock().unwrap().build()?);
    let user_account_infrastructure_di = Arc::new(user_account_infrastructure_builder.build()?);
    let oidc_infrastructure_di = oidc_infrastructure_builder.build().await?;

    let router = Arc::new(RouterCombinator::new(
        vec![
            user_infrastructure_di.api_di.router.clone(),
            user_account_infrastructure_di.api_di.router.clone(),
            oidc_infrastructure_di.api_di.router.clone(),
        ],
        rest_di.error_pre_handler.clone(),
    ));

    let server = Arc::new(Server::new(
        addr,
        router,
        rest_di.response_transformer.clone(),
        rest_di.error_handler.clone(),
        logger,
    ));

    Ok(Di::new(server))
}
