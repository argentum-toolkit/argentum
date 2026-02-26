use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_log_business::LoggerTrait;
use argentum_notification_business::mock::StdoutNotifier;
use argentum_oidc_infrastructure::default_di::OidcInfrastructureDiBuilder;
use argentum_rest_infrastructure::{
    di::RestDiBuilder,
    service::{RouterCombinator, Server},
};

use argentum_standard_business::error::DiError;
use argentum_standard_infrastructure::data_type::unique_id::{UniqueId, UniqueIdFactory};
use argentum_user_account_infrastructure::di::UserAccountInfrastructureDiBuilder;
use argentum_user_business::di::UserBusinessDiBuilder;
use argentum_user_infrastructure::di::UserInfrastructureDiBuilder;
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

pub struct SkeletonDi<L>
where
    L: LoggerTrait,
{
    // Public services
    pub server: Arc<Server<L>>,
}

pub struct SkeletonDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    pub logger: Arc<L>,
    pub rest_builder: Arc<RestDiBuilder<UniqueId, L>>,
    pub user_infrastructure_builder: Arc<Mutex<UserInfrastructureDiBuilder<L>>>,
    pub user_account_infrastructure_builder: Arc<UserAccountInfrastructureDiBuilder<L>>,
    pub oidc_infrastructure_builder: Arc<OidcInfrastructureDiBuilder<L>>,
}

impl<L> SkeletonDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    pub async fn try_new(logger: Arc<L>) -> Result<Self, DiError> {
        let env: Env = envy::from_env::<Env>().map_err(|e| format!("Cant load env. Error: {e}"))?;

        let unique_id_factory = Arc::new(UniqueIdFactory::new());

        let client = reqwest::Client::builder()
            .user_agent("ArgentumToolKit/0.4")
            .pool_max_idle_per_host(20)
            .build()
            .map_err(|e| format!("Cant create reqwest client. Error: {e}"))?;
        let r_client = Arc::new(client);

        let user_business_builder =
            Arc::new(Mutex::new(UserBusinessDiBuilder::new(logger.clone())));
        let rest_builder = Arc::new(RestDiBuilder::new(
            user_business_builder.clone(),
            logger.clone(),
        ));

        let tmp_user_builder = UserInfrastructureDiBuilder::new(
            &env.ag_user_database_url,
            10,
            user_business_builder.clone(),
            rest_builder.clone(),
            logger.clone(),
        );
        let user_infrastructure_builder =
            Arc::new(Mutex::new(tmp_user_builder.with_default_services().await?));

        let notifier = Arc::new(StdoutNotifier::new());

        let pbkdf2 = Arc::new(Pbkdf2::new());

        let tmp_user_account_builder = UserAccountInfrastructureDiBuilder::new(
            "Argentum ToolKit. Example of API application",
            3600, // TTL 1h
            "http://localhost:8082/change-password/",
            &env.ag_user_account_database_url,
            10,
            &env.ag_user_account_member_jwt_secret,
            10 * 3600,
            user_business_builder.clone(),
            rest_builder.clone(),
            unique_id_factory.clone(),
            logger.clone(),
            notifier,
            pbkdf2.clone(),
            pbkdf2.clone(),
        );
        let user_account_infrastructure_builder =
            Arc::new(tmp_user_account_builder.with_default_services().await?);

        let tmp_oidc_infrastructure_builder = OidcInfrastructureDiBuilder::new(
            &env.ag_oidc_database_url,
            10,
            &env.github_client_id,
            &env.github_client_secret,
            user_business_builder.clone(),
            user_account_infrastructure_builder.business_builder.clone(),
            rest_builder.clone(),
            unique_id_factory.clone(),
            logger.clone(),
            r_client.clone(),
        );

        let oidc_infrastructure_builder = Arc::new(
            tmp_oidc_infrastructure_builder
                .with_default_services()
                .await?,
        );

        Ok(Self {
            logger,
            rest_builder,
            user_infrastructure_builder,
            user_account_infrastructure_builder,
            oidc_infrastructure_builder,
        })
    }

    pub async fn build(&self) -> Result<SkeletonDi<L>, DiError> {
        let rest_di = self.rest_builder.build()?;
        let user_infrastructure_di = self.user_infrastructure_builder.lock()?.build()?;
        let user_account_infrastructure_di = self.user_account_infrastructure_builder.build()?;
        let oidc_infrastructure_di = self.oidc_infrastructure_builder.build().await?;

        let router = Arc::new(RouterCombinator::new(
            vec![
                user_infrastructure_di.api_di.router.clone(),
                user_account_infrastructure_di.api_di.router.clone(),
                oidc_infrastructure_di.api_di.router.clone(),
            ],
            rest_di.error_pre_handler.clone(),
        ));

        // let listen = "172.18.0.1:8088";
        // let listen = "127.0.0.1:8088";
        let listen = "0.0.0.0:8088";
        let addr: SocketAddr = listen
            .parse()
            .map_err(|e| format!("Unable to parse socket address. Error: {e}"))?;

        let server = Arc::new(Server::new(
            addr,
            router,
            rest_di.response_transformer.clone(),
            rest_di.error_handler.clone(),
            self.logger.clone(),
        ));

        Ok(SkeletonDi { server })
    }
}
