use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_event_business::di::EventBusinessDiBuilder;
use argentum_log_business::LoggerTrait;
use argentum_notification_business::mock::StdoutNotifier;
use argentum_oidc_infrastructure::default_di::OidcInfrastructureDiBuilder;
use argentum_rest_infrastructure::{
    di::RestDiBuilder,
    service::{
        RouterCombinator, RouterTrait,
        server::{Http1Server, Http2Server},
    },
};

use argentum_standard_business::error::DiError;
use argentum_standard_infrastructure::data_type::unique_id::{UniqueId, UniqueIdFactory};
use argentum_user_account_infrastructure::di::UserAccountInfrastructureDiBuilder;
use argentum_user_business::di::UserBusinessDiBuilder;
use argentum_user_infrastructure::di::UserInfrastructureDiBuilder;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Env {
    pub ag_github_client_id: String,
    pub ag_github_client_secret: String,
    pub ag_google_client_id: String,
    pub ag_google_client_secret: String,
    pub ag_oidc_database_url: String,
    pub ag_user_database_url: String,
    pub ag_user_account_database_url: String,
    pub ag_user_account_member_jwt_secret: String,
    pub ag_user_agent: String,
    pub ag_product_name: String,
    pub ag_email_confirmation_front_url: String,
    pub ag_restore_password_front_url: String,
    pub tls_cert_file: String,
    pub tls_key_file: String,
}

pub struct SkeletonApiDi<L>
where
    L: LoggerTrait,
{
    // Public services
    pub http1_server: Arc<Http1Server<L>>,
    pub http2_server: Arc<Http2Server<L>>,
}

pub struct SkeletonApiDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    listen: String,
    pub logger: Arc<L>,
    pub event_business_builder: Arc<EventBusinessDiBuilder>,
    pub rest_builder: Arc<RestDiBuilder<UniqueId, L>>,
    pub user_infrastructure_builder: Arc<UserInfrastructureDiBuilder<L>>,
    pub user_account_infrastructure_builder: Arc<UserAccountInfrastructureDiBuilder<L>>,
    pub oidc_infrastructure_builder: Arc<OidcInfrastructureDiBuilder<L>>,
}

impl<L> SkeletonApiDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    pub fn try_new(listen: &str, logger: Arc<L>) -> Result<Self, DiError> {
        let env: Env = envy::from_env::<Env>().map_err(|e| format!("Cant load env. Error: {e}"))?;

        let unique_id_factory = Arc::new(UniqueIdFactory::new());

        let client = reqwest::Client::builder()
            .user_agent(&env.ag_user_agent)
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
        let user_infrastructure_builder = Arc::new(tmp_user_builder.with_default_services()?);

        let notifier = Arc::new(StdoutNotifier::new());

        let pbkdf2 = Arc::new(Pbkdf2::new());

        let event_business_builder = Arc::new(EventBusinessDiBuilder::new());

        let tmp_user_account_builder = UserAccountInfrastructureDiBuilder::new(
            &env.ag_product_name,
            3600 * 24 * 30,
            3600, // TTL 1h
            &env.ag_email_confirmation_front_url,
            &env.ag_restore_password_front_url,
            &env.ag_user_account_database_url,
            10,
            &env.ag_user_account_member_jwt_secret,
            10 * 3600,
            user_business_builder.clone(),
            event_business_builder.clone(),
            rest_builder.clone(),
            unique_id_factory.clone(),
            logger.clone(),
            notifier,
            pbkdf2.clone(),
            pbkdf2.clone(),
        );
        let user_account_infrastructure_builder =
            Arc::new(tmp_user_account_builder.with_default_services()?);

        let tmp_oidc_infrastructure_builder = OidcInfrastructureDiBuilder::new(
            &env.ag_oidc_database_url,
            10,
            &env.ag_github_client_id,
            &env.ag_github_client_secret,
            &env.ag_google_client_id,
            &env.ag_google_client_secret,
            event_business_builder.clone(),
            user_business_builder.clone(),
            user_account_infrastructure_builder.business_builder.clone(),
            rest_builder.clone(),
            unique_id_factory.clone(),
            logger.clone(),
            r_client.clone(),
        );

        let oidc_infrastructure_builder =
            Arc::new(tmp_oidc_infrastructure_builder.with_default_services()?);

        Ok(Self {
            event_business_builder,
            listen: listen.into(),
            logger,
            rest_builder,
            user_infrastructure_builder,
            user_account_infrastructure_builder,
            oidc_infrastructure_builder,
        })
    }

    pub fn build(&self, routers: Vec<Arc<dyn RouterTrait>>) -> Result<SkeletonApiDi<L>, DiError> {
        let env: Env = envy::from_env::<Env>().map_err(|e| format!("Cant load env. Error: {e}"))?;

        let rest_di = self.rest_builder.build()?;
        let user_infrastructure_di = self.user_infrastructure_builder.build()?;
        let user_account_infrastructure_di = self.user_account_infrastructure_builder.build()?;
        let oidc_infrastructure_di = self.oidc_infrastructure_builder.build()?;

        let mut r: Vec<Arc<dyn RouterTrait>> = vec![
            user_infrastructure_di.api_di.router.clone(),
            user_account_infrastructure_di.api_di.router.clone(),
            oidc_infrastructure_di.api_di.router.clone(),
        ];
        r.extend(routers);

        let router = Arc::new(RouterCombinator::new(r, rest_di.error_pre_handler.clone()));

        let addr: SocketAddr = self
            .listen
            .parse()
            .map_err(|e| format!("Unable to parse socket address. Error: {e}"))?;

        let http1_server = Arc::new(Http1Server::new(
            addr,
            router.clone(),
            rest_di.response_transformer.clone(),
            rest_di.error_handler.clone(),
            self.logger.clone(),
        ));

        let http2_server = Arc::new(Http2Server::new(
            addr,
            &env.tls_cert_file,
            &env.tls_key_file,
            router,
            rest_di.response_transformer.clone(),
            rest_di.error_handler.clone(),
            self.logger.clone(),
        ));

        Ok(SkeletonApiDi {
            http1_server,
            http2_server,
        })
    }
}
