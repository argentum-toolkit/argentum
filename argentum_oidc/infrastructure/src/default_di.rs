use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_oidc_business::di::{OidcBusinessDi, OidcBusinessDiBuilder};
use argentum_oidc_rest::ApiDiC;
use argentum_rest_infrastructure::di::RestDiBuilder;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::di::UserAccountBusinessDiBuilder;

use argentum_user_business::di::UserBusinessDiBuilder;
use sqlx::postgres::PgPoolOptions;

use std::sync::{Arc, Mutex};

use crate::{
    db::repository::CredentialRepository, github::GitHubProvider,
    rest::handler::ExchangeCodeHandler,
};

pub struct OidcInfrastructureDi<L>
where
    L: LoggerTrait,
{
    // Public services
    pub business_di: OidcBusinessDi<UniqueId, L>,
    pub api_di: ApiDiC<UniqueId>,
}

pub struct OidcInfrastructureDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    connection_url: String,
    max_db_connections: u32,
    github_client_id: String,
    github_client_secret: String,

    business_builder: Arc<Mutex<OidcBusinessDiBuilder<UniqueId, L>>>,
    rest_builder: Arc<RestDiBuilder<UniqueId, L>>,

    logger: Arc<L>,
    client: Arc<reqwest::Client>,
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

            business_builder: Arc::new(Mutex::new(OidcBusinessDiBuilder::new(
                user_business_builder.clone(),
                user_account_business_builder.clone(),
                id_factory.clone(),
                logger.clone(),
            ))),

            rest_builder,
            logger,
            client,
        }
    }

    pub async fn with_default_services(&mut self) -> Result<&mut Self, String> {
        let pool = PgPoolOptions::new()
            .max_connections(self.max_db_connections)
            .min_connections(2)
            .connect(&self.connection_url)
            .await
            .map_err(|e| format!("Can't create PG connection pool. Err: {e}"))?;

        let pg_adapter = Arc::new(SqlxPostgresAdapter::new(pool, self.logger.clone()));

        let provider = Arc::new(GitHubProvider::new(
            &self.github_client_id,
            &self.github_client_secret,
            self.client.clone(),
            self.logger.clone(),
        ));

        self.business_builder
            .lock()
            .unwrap()
            .with_credential_repository(Arc::new(CredentialRepository::new(pg_adapter)))
            .with_provider(provider);

        Ok(self)
    }

    pub async fn build(&mut self) -> Result<OidcInfrastructureDi<L>, String> {
        let business_di = self.business_builder.lock().unwrap().build()?;

        let exchange_code_handler = Arc::new(ExchangeCodeHandler::new(
            business_di.user_exchanges_code_to_token_uc.clone(),
        ));

        let rest_di = self.rest_builder.build();

        let api_di = ApiDiC::new(
            "/api/v1".to_string(),
            rest_di.request_transformer.clone(),
            rest_di.bearer_authenticator,
            exchange_code_handler,
            rest_di.error_pre_handler.clone(),
        );

        Ok(OidcInfrastructureDi {
            business_di,
            api_di,
        })
    }
}
