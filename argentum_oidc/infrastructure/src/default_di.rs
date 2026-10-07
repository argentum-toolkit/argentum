use crate::{
    db::OidcSqlxPostgresAdapter, db::repository::CredentialRepository, github::GitHubProvider,
    google::GoogleProvider, rest::handler::ExchangeCodeHandler,
};

use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;

use argentum_log_business::LoggerTrait;
use argentum_oidc_business::service::provider::ProviderStrategyTrait;
use argentum_oidc_business::{
    repository::CredentialRepositoryTrait, use_case::UserExchangesCodeToTokenUc,
};
use argentum_oidc_rest::{oidc_api_provide, server::handler::ExchangeCodeTrait};
use argentum_standard_business::di_container::{Container, DiError};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use sqlx::postgres::PgPoolOptions;

use std::{collections::HashMap, sync::Arc};

pub async fn oidc_infrastructure_provider<L>(
    container: &mut Container,
    connection_url: impl Into<String>,
    max_db_connections: u32,
    github_client_id: impl Into<String>,
    github_client_secret: impl Into<String>,
    google_client_id: impl Into<String>,
    google_client_secret: impl Into<String>,
    client: Arc<reqwest::Client>,
) -> Result<(), DiError>
where
    L: LoggerTrait + 'static,
{
    oidc_api_provide::<UniqueId>(container, "/api/v1");

    let connection_url = connection_url.into();
    let github_client_id = github_client_id.into();
    let github_client_secret = github_client_secret.into();
    let google_client_id = google_client_id.into();
    let google_client_secret = google_client_secret.into();
    let pool = PgPoolOptions::new()
        .max_connections(max_db_connections)
        .min_connections(2)
        .connect(&connection_url)
        .await
        .map_err(anyhow::Error::from)?;

    container.provide_factory::<OidcSqlxPostgresAdapter<L>, _>(move |c| {
        let logger = c.resolve::<L>()?;

        Ok(Arc::new(OidcSqlxPostgresAdapter(SqlxPostgresAdapter::new(
            pool.clone(),
            logger,
        ))))
    });

    container.provide_factory::<dyn CredentialRepositoryTrait<UserId = UniqueId>, _>(|c| {
        let adapter = c.resolve::<OidcSqlxPostgresAdapter<L>>()?;
        Ok(Arc::new(CredentialRepository::new(adapter)))
    });

    container.provide_factory::<GitHubProvider<L>, _>({
        let client = client.clone();
        move |c| {
            let logger = c.resolve::<L>()?;
            Ok(Arc::new(GitHubProvider::new(
                &github_client_id,
                &github_client_secret,
                client.clone(),
                logger,
            )))
        }
    });

    container.provide_factory::<GoogleProvider<L>, _>({
        let client = client.clone();
        move |c| {
            let logger = c.resolve::<L>()?;
            Ok(Arc::new(GoogleProvider::new(
                &google_client_id,
                &google_client_secret,
                client.clone(),
                logger,
            )))
        }
    });

    container.provide_factory::<HashMap<String, Arc<dyn ProviderStrategyTrait>>, _>(|c| {
        let github_provider = c.resolve::<GitHubProvider<L>>()?;
        let google_provider = c.resolve::<GoogleProvider<L>>()?;

        let mut providers: HashMap<String, Arc<dyn ProviderStrategyTrait>> = HashMap::new();
        providers.insert("GitHub".into(), github_provider);
        providers.insert("Google".into(), google_provider);

        Ok(Arc::new(providers))
    });

    container.provide_factory::<dyn ExchangeCodeTrait<Id = UniqueId>, _>(|c| {
        let uc = c.resolve::<UserExchangesCodeToTokenUc<UniqueId, L>>()?;
        Ok(Arc::new(ExchangeCodeHandler::new(uc)))
    });
    Ok(())
}
