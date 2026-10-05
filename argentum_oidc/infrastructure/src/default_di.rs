use crate::{
    db::repository::CredentialRepository, github::GitHubProvider, google::GoogleProvider,
    rest::handler::ExchangeCodeHandler,
};

use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_event_business::di::EventBusinessDiBuilder;
use argentum_log_business::LoggerTrait;
use argentum_oidc_business::{
    di::{OidcBusinessDi, OidcBusinessDiBuilder},
    service::provider::ProviderStrategyTrait,
};
use argentum_oidc_rest::ApiDi;
use argentum_rest_infrastructure::di::RestDiBuilder;
use argentum_standard_business::{data_type::id::IdFactory, error::DiError};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::di::UserAccountBusinessDiBuilder;

use argentum_user_business::di::UserBusinessDiBuilder;
use sqlx::postgres::PgPoolOptions;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::runtime::Runtime;

pub struct OidcInfrastructureDi<L>
where
    L: LoggerTrait,
{
    pub business_di: Arc<OidcBusinessDi<UniqueId, L>>,
    pub api_di: ApiDi<UniqueId>,
}

pub struct OidcInfrastructureDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    connection_url: String,
    max_db_connections: u32,
    github_client_id: String,
    github_client_secret: String,
    google_client_id: String,
    google_client_secret: String,

    business_builder: Arc<Mutex<OidcBusinessDiBuilder<UniqueId, L>>>,
    rest_builder: Arc<RestDiBuilder<UniqueId, L>>,

    logger: Arc<L>,
    client: Arc<reqwest::Client>,

    built_di: Mutex<Option<Arc<OidcInfrastructureDi<L>>>>,
}

impl<L> OidcInfrastructureDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    pub fn new(
        connection_url: &str,
        max_db_connections: u32,
        github_client_id: &str,
        github_client_secret: &str,
        google_client_id: &str,
        google_client_secret: &str,

        event_business_builder: Arc<EventBusinessDiBuilder>,
        user_business_builder: Arc<Mutex<UserBusinessDiBuilder<UniqueId, L>>>,
        user_account_business_builder: Arc<Mutex<UserAccountBusinessDiBuilder<UniqueId, L>>>,
        rest_builder: Arc<RestDiBuilder<UniqueId, L>>,

        id_factory: Arc<dyn IdFactory<Id = UniqueId>>,
        logger: Arc<L>,
        client: Arc<reqwest::Client>,
    ) -> Self {
        Self {
            connection_url: connection_url.to_string(),
            max_db_connections,
            github_client_id: github_client_id.to_string(),
            github_client_secret: github_client_secret.to_string(),
            google_client_id: google_client_id.to_string(),
            google_client_secret: google_client_secret.to_string(),

            business_builder: Arc::new(Mutex::new(OidcBusinessDiBuilder::new(
                event_business_builder,
                user_business_builder.clone(),
                user_account_business_builder.clone(),
                id_factory.clone(),
                logger.clone(),
            ))),

            rest_builder,
            logger,
            client,

            built_di: Mutex::new(None),
        }
    }

    pub fn with_default_services(self) -> Result<Self, DiError> {
        let pool = Runtime::new()
            .map_err(|e| DiError::Oher(e.to_string()))?
            .block_on(
                PgPoolOptions::new()
                    .max_connections(self.max_db_connections)
                    .min_connections(2)
                    .connect(&self.connection_url),
            )
            .map_err(|e| {
                format!(
                    "OIDC COMPONENT: Can't create PG connection pool for connection `{}`. Error: {e}",
                    &self.connection_url
                )
            })?;

        let pg_adapter = Arc::new(SqlxPostgresAdapter::new(pool, self.logger.clone()));

        let github_provider = Arc::new(GitHubProvider::new(
            &self.github_client_id,
            &self.github_client_secret,
            self.client.clone(),
            self.logger.clone(),
        ));

        let google_provider = Arc::new(GoogleProvider::new(
            &self.google_client_id,
            &self.google_client_secret,
            self.client.clone(),
            self.logger.clone(),
        ));

        let mut providers: HashMap<String, Arc<dyn ProviderStrategyTrait>> = HashMap::new();

        providers.insert("GitHub".into(), github_provider);
        providers.insert("Google".into(), google_provider);

        self.business_builder
            .lock()?
            .with_credential_repository(Arc::new(CredentialRepository::new(pg_adapter)))
            .with_provider_strategies(providers);

        Ok(self)
    }

    pub fn build(&self) -> Result<Arc<OidcInfrastructureDi<L>>, DiError> {
        if let Some(di) = self.built_di.lock()?.clone() {
            return Ok(di);
        }

        let business_di = self.business_builder.lock()?.build()?;

        let exchange_code_handler = Arc::new(ExchangeCodeHandler::new(
            business_di.user_exchanges_code_to_token_uc.clone(),
        ));

        let rest_di = self.rest_builder.build()?;

        let api_di = ApiDi::new(
            "/api/v1",
            rest_di.request_transformer.clone(),
            rest_di.bearer_authenticator.clone(),
            exchange_code_handler,
            rest_di.error_pre_handler.clone(),
        );

        let di = Arc::new(OidcInfrastructureDi {
            business_di,
            api_di,
        });

        *self.built_di.lock()? = Some(di.clone());

        Ok(di)
    }
}
