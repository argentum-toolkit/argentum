use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_log_business::LoggerTrait;
use argentum_oidc_business::{
    di::{OidcBusinessDiC, OidcBusinessDiCBuilder},
    repository::CredentialRepositoryTrait,
    service::code_exchanger::DefaultCodeExchanger,
};
use argentum_oidc_rest::server::handler::ExchangeCodeTrait;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::token::GeneratorTrait;
use argentum_user_business::{
    repository::{
        session_repository::SessionRepositoryTrait,
        user_repository::AuthenticatedUserRepositoryTrait,
    },
    service::GuestBinder,
};
use sqlx::postgres::PgPoolOptions;

use std::sync::Arc;

use crate::{
    db::repository::CredentialRepository, github::GitHubProvider,
    rest::handler::ExchangeCodeHandler,
};

pub struct OidcInfrastructureDiC<L>
where
    L: LoggerTrait,
{
    pub business_dic: OidcBusinessDiC<UniqueId, L>,

    // Public services
    pub exchange_code_handler: Arc<dyn ExchangeCodeTrait<Id = UniqueId>>,
}

pub struct OidcInfrastructureDiCBuilder<L>
where
    L: LoggerTrait + 'static,
{
    business_builder: OidcBusinessDiCBuilder<UniqueId, L>,
    session_repository: Arc<dyn SessionRepositoryTrait<Id = UniqueId>>,
    token_generator: Arc<dyn GeneratorTrait<Id = UniqueId>>,
    authenticated_user_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = UniqueId>>,
    guest_binder: Arc<GuestBinder<UniqueId, L>>,
    id_factory: Arc<dyn IdFactory<Id = UniqueId>>,

    logger: Arc<L>,
    client: Arc<reqwest::Client>,
}

impl<L> OidcInfrastructureDiCBuilder<L>
where
    L: LoggerTrait + 'static,
{
    pub fn new(
        session_repository: Arc<dyn SessionRepositoryTrait<Id = UniqueId>>,
        token_generator: Arc<dyn GeneratorTrait<Id = UniqueId>>,
        authenticated_user_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = UniqueId>>,
        guest_binder: Arc<GuestBinder<UniqueId, L>>,
        id_factory: Arc<dyn IdFactory<Id = UniqueId>>,
        logger: Arc<L>,
        client: Arc<reqwest::Client>,
    ) -> Self {
        Self {
            business_builder: OidcBusinessDiCBuilder::new(),
            session_repository,
            token_generator,
            authenticated_user_repository,
            guest_binder,
            id_factory,
            logger,
            client,
        }
    }

    pub async fn build(
        &mut self,
        connection_url: &str,
        max_db_connections: u32,
        github_client_id: &str,
        github_client_secret: &str,
        logger: Arc<L>,
    ) -> Result<OidcInfrastructureDiC<L>, String> {
        let pool = PgPoolOptions::new()
            .max_connections(max_db_connections)
            .min_connections(2)
            .connect(connection_url)
            .await
            .map_err(|e| format!("Can't create PG connection pool. Err: {e}"))?;

        let pg_adapter = Arc::new(SqlxPostgresAdapter::new(pool, logger));

        let credential_repository: Arc<dyn CredentialRepositoryTrait<UserId = UniqueId>> =
            Arc::new(CredentialRepository::new(pg_adapter));

        let provider = Arc::new(GitHubProvider::new(
            github_client_id,
            github_client_secret,
            self.client.clone(),
            self.logger.clone(),
        ));

        let gh_code_exchanger = Arc::new(DefaultCodeExchanger::new(
            provider,
            self.authenticated_user_repository.clone(),
            credential_repository.clone(),
            self.session_repository.clone(),
            self.token_generator.clone(),
            self.id_factory.clone(),
            self.logger.clone(),
        ));

        let business_dic = self
            .business_builder
            .with_auth_code_exchanger(gh_code_exchanger.clone())
            .with_credential_repository(credential_repository)
            .with_guest_binder(self.guest_binder.clone())
            .build()?;

        let exchange_code_handler = Arc::new(ExchangeCodeHandler::new(
            business_dic.user_exchanges_code_to_token_uc.clone(),
        ));

        Ok(OidcInfrastructureDiC {
            business_dic,
            exchange_code_handler,
        })
    }
}
