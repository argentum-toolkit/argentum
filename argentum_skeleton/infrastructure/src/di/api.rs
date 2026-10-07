use std::sync::Arc;

use argentum_encryption_business::password::{Encryptor, Validator};
use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_event_business::di::event_business_provide;
use argentum_log_business::LoggerTrait;
use argentum_oidc_infrastructure::{
    default_di::oidc_infrastructure_provider, oidc_business_provide,
};
use argentum_rest_infrastructure::{
    di::rest_infrastructure_provide,
    service::{
        ErrorHandler, ErrorPreHandler, ResponseToJsonTransformer, RouterCombinator, RouterTrait,
        server::{Http1Server, Http2Server},
    },
};

use argentum_standard_business::di_container::{Container, DiError as ContainerDiError};
use argentum_standard_business::error::DiError;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::di::user_account_business_provide;
use argentum_user_account_infrastructure::di::user_account_infrastructure_provider;
use argentum_user_business::di::user_business_provide;
use argentum_user_infrastructure::di::user_infrastructure_provide;
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

pub async fn skeleton_api_provide<L>(
    container: &mut Container,
    listen: &'static str,
) -> Result<(), DiError>
where
    L: LoggerTrait + 'static,
{
    let env: Env = envy::from_env::<Env>().map_err(|e| format!("Cant load env. Error: {e}"))?;

    event_business_provide(container);

    rest_infrastructure_provide::<UniqueId, L>(container);

    user_business_provide::<UniqueId, L>(container);

    user_account_business_provide::<UniqueId, L>(
        container,
        env.ag_product_name.clone(),
        3600 * 24 * 30, // TTL 30 days
        3600,           // TTL 1h
        env.ag_email_confirmation_front_url.clone(),
        env.ag_restore_password_front_url.clone(),
    );

    oidc_business_provide::<UniqueId, L>(container);

    user_infrastructure_provide::<UniqueId, L>(container, env.ag_user_database_url.clone(), 10)
        .await?;
    user_account_infrastructure_provider::<UniqueId, L>(
        container,
        env.ag_user_account_database_url.clone(),
        10,
        &env.ag_user_account_member_jwt_secret,
        10 * 3600,
    )
    .await?;

    let client = reqwest::Client::builder()
        .user_agent(&env.ag_user_agent)
        .pool_max_idle_per_host(20)
        .build()
        .map_err(|e| format!("Cant create reqwest client. Error: {e}"))?;

    oidc_infrastructure_provider::<L>(
        container,
        env.ag_oidc_database_url.clone(),
        10,
        env.ag_github_client_id.clone(),
        env.ag_github_client_secret.clone(),
        env.ag_google_client_id.clone(),
        env.ag_google_client_secret.clone(),
        Arc::new(client),
    )
    .await?;

    let pbkdf2 = Arc::new(Pbkdf2::new());
    let encryptor = pbkdf2.clone();

    container.provide_factory::<dyn Encryptor, _>(move |_c| Ok(encryptor.clone()));
    container.provide_factory::<dyn Validator, _>(move |_c| Ok(pbkdf2.clone()));

    container.provide_factory::<Http1Server<L>, _>(move |c| {
        let logger = c.resolve::<L>()?;

        let pre_handler = c.resolve::<ErrorPreHandler>()?;

        let user_router = c.resolve::<argentum_user_rest::server::Router<UniqueId>>()?;
        let user_account_router =
            c.resolve::<argentum_user_account_rest::server::Router<UniqueId>>()?;
        let oidc_router = c.resolve::<argentum_oidc_rest::server::Router<UniqueId>>()?;

        //TODO: inject all services
        let r: Vec<Arc<dyn RouterTrait>> = vec![user_router, user_account_router, oidc_router];
        // r.extend(routers);

        let router = Arc::new(RouterCombinator::new(r, pre_handler));
        // let router = container.resolve::<RouterCombinator>()?;
        let response_transformer = c.resolve::<ResponseToJsonTransformer>()?;
        let error_handler = c.resolve::<ErrorHandler<L>>()?;
        let addr = listen.parse().map_err(|e| {
            ContainerDiError::Other(format!("Unable to parse socket address. Error: {e}"))
        })?;

        Ok(Arc::new(Http1Server::new(
            addr,
            router,
            response_transformer,
            error_handler,
            logger,
        )))
    });

    container.provide_factory::<Http2Server<L>, _>(move |c| {
        let logger = c.resolve::<L>()?;

        // let pre_handler = c.resolve::<ErrorPreHandler>>()?;

        // let router = Arc::new(RouterCombinator::new(r, pre_handler));

        let router = c.resolve::<RouterCombinator>()?;
        let response_transformer = c.resolve::<ResponseToJsonTransformer>()?;
        let error_handler = c.resolve::<ErrorHandler<L>>()?;

        let addr = listen.parse().map_err(|e| {
            ContainerDiError::Other(format!("Unable to parse socket address. Error: {e}"))
        })?;

        Ok(Arc::new(Http2Server::new(
            addr,
            &env.tls_cert_file,
            &env.tls_key_file,
            router,
            response_transformer,
            error_handler,
            logger,
        )))
    });

    Ok(())
}
