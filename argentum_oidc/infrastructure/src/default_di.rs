use crate::{
    db::OidcSqlxPostgresAdapter, db::repository::CredentialRepository, github::GitHubProvider,
    google::GoogleProvider, rest::handler::ExchangeCodeHandler,
};

use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
use argentum_event_business::EventDispatcher;

use argentum_log_business::LoggerTrait;
use argentum_oidc_business::service::provider::ProviderStrategyTrait;
use argentum_oidc_business::service::redirect_url::OidcRedirectUrlResolver;
use argentum_oidc_business::{
    listener::MemberDeletedAccountListener, repository::CredentialRepositoryTrait,
    use_case::UserExchangesCodeToTokenUc,
};
use argentum_oidc_rest::{oidc_rest_provide, server::handler::ExchangeCodeTrait};
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
    redirect_url: impl Into<String>,
    client: Arc<reqwest::Client>,
) -> Result<(), DiError>
where
    L: LoggerTrait + 'static,
{
    oidc_rest_provide::<UniqueId>(container, "/api/v1");

    let connection_url = connection_url.into();
    let github_client_id = github_client_id.into();
    let github_client_secret = github_client_secret.into();
    let google_client_id = google_client_id.into();
    let google_client_secret = google_client_secret.into();
    let redirect_url = redirect_url.into();
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

    {
        let event_dispatcher = container.resolve::<EventDispatcher>()?;
        let credential_repository =
            container.resolve::<dyn CredentialRepositoryTrait<UserId = UniqueId>>()?;
        let logger = container.resolve::<L>()?;

        event_dispatcher
            .register_listener(Arc::new(MemberDeletedAccountListener::new(
                credential_repository,
                logger,
            )))
            .map_err(|e| DiError::with_context("Can't register OIDC member listener", e))?;
    }

    container.provide_factory::<OidcRedirectUrlResolver, _>(move |_c| {
        Ok(Arc::new(OidcRedirectUrlResolver::new(&redirect_url)))
    });

    container.provide_factory::<GitHubProvider<L>, _>({
        let client = client.clone();
        move |c| {
            let logger = c.resolve::<L>()?;
            let redirect_url_resolver = c.resolve::<OidcRedirectUrlResolver>()?;
            Ok(Arc::new(GitHubProvider::new(
                &github_client_id,
                &github_client_secret,
                redirect_url_resolver,
                client.clone(),
                logger,
            )))
        }
    });

    container.provide_factory::<GoogleProvider<L>, _>({
        let client = client.clone();
        move |c| {
            let logger = c.resolve::<L>()?;
            let redirect_url_resolver = c.resolve::<OidcRedirectUrlResolver>()?;
            Ok(Arc::new(GoogleProvider::new(
                &google_client_id,
                &google_client_secret,
                redirect_url_resolver,
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
