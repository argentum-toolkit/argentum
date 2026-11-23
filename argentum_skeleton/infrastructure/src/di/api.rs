use std::sync::Arc;

use argentum_captcha_business::di::{captcha_business_provide, captcha_business_provide_mock};
use argentum_captcha_infrastructure::di::captcha_infrastructure_provide_turnstile;
use argentum_contact_us_business::di::contact_us_business_provide;
use argentum_contact_us_infrastructure::di::contact_us_infrastructure_provider;
use argentum_contact_us_infrastructure::notifier::build_smtp_transport;
use argentum_encryption_business::password::{Encryptor, Validator};
use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_event_business::di::event_business_provide;
use argentum_log_business::LoggerTrait;
use argentum_log_infrastructure::log_bridge::LogBridge;
use argentum_notification_business::{NotifierTrait, mock::StdoutNotifier};
use argentum_notification_infrastructure::email_notifier::EmailNotifier;
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

use argentum_standard_business::{
    data_type::id::IdFactory,
    di_container::{Container, DiError},
};
use argentum_standard_infrastructure::data_type::unique_id::{UniqueId, UniqueIdFactory};
use argentum_user_account_business::di::user_account_business_provide;
use argentum_user_business::repository::user_repository::MemberRepositoryTrait;

use argentum_user_account_infrastructure::di::user_account_infrastructure_provider;
use argentum_user_business::di::user_business_provide;
use argentum_user_infrastructure::di::user_infrastructure_provide;
use secrecy::SecretString;
use serde::Deserialize;

fn default_password_min_length() -> usize {
    8
}

fn default_password_complexity_enabled() -> bool {
    true
}

fn default_captcha_enabled() -> bool {
    true
}

fn default_smtp_port() -> u16 {
    587
}

fn default_smtp_tls_mode() -> String {
    "starttls".to_string()
}

fn default_email_from() -> String {
    "Argentum <noreply@argentum-toolkit.com>".to_string()
}

#[derive(Deserialize, Debug)]
struct Env {
    pub ag_github_client_id: String,
    pub ag_github_client_secret: String,
    pub ag_google_client_id: String,
    pub ag_google_client_secret: String,
    pub ag_oidc_redirect_url: String,
    pub ag_oidc_database_url: String,
    pub ag_user_database_url: String,
    pub ag_user_account_database_url: String,
    pub ag_contact_us_database_url: String,
    #[serde(default)]
    pub ag_contact_us_admin_email: String,
    #[serde(default)]
    pub ag_smtp_host: String,
    #[serde(default = "default_smtp_port")]
    pub ag_smtp_port: u16,
    /// `tls`, `starttls` (default) or `none` (e.g. for a local MailHog)
    #[serde(default = "default_smtp_tls_mode")]
    pub ag_smtp_tls_mode: String,
    #[serde(default)]
    pub ag_smtp_username: String,
    #[serde(default)]
    pub ag_smtp_password: String,
    #[serde(default = "default_email_from")]
    pub ag_email_from: String,
    pub ag_user_account_member_jwt_secret: String,
    pub ag_user_agent: String,
    pub ag_product_name: String,
    pub ag_email_confirmation_front_url: String,
    pub ag_restore_password_front_url: String,
    #[serde(default = "default_password_min_length")]
    pub ag_password_min_length: usize,
    #[serde(default = "default_password_complexity_enabled")]
    pub ag_password_complexity_enabled: bool,
    #[serde(default = "default_captcha_enabled")]
    pub ag_captcha_enabled: bool,
    #[serde(default)]
    pub ag_captcha_turnstile_secret_key: String,
    pub tls_cert_file: String,
    pub tls_key_file: String,
}

fn resolve_router(container: &Container) -> Result<Arc<dyn RouterTrait>, DiError> {
    let pre_handler = container.resolve::<ErrorPreHandler>()?;

    let user_router = container.resolve::<argentum_user_rest::server::Router<UniqueId>>()?;
    let user_account_router =
        container.resolve::<argentum_user_account_rest::server::Router<UniqueId>>()?;
    let oidc_router = container.resolve::<argentum_oidc_rest::server::Router<UniqueId>>()?;
    let contact_us_router =
        container.resolve::<argentum_contact_us_rest::server::Router<UniqueId>>()?;

    let routers: Vec<Arc<dyn RouterTrait>> = vec![
        user_router,
        user_account_router,
        oidc_router,
        contact_us_router,
    ];

    Ok(Arc::new(RouterCombinator::new(routers, pre_handler)))
}

pub async fn skeleton_api_provide<L>(
    container: &mut Container,
    listen: &'static str,
) -> Result<(), DiError>
where
    L: LoggerTrait + 'static,
{
    let logger: Arc<L> = container.resolve::<L>()?;
    LogBridge::install(logger);

    let env: Env =
        envy::from_env::<Env>().map_err(|e| DiError::with_context("Can't load environment", e))?;

    container.provide_factory::<dyn IdFactory<Id = UniqueId>, _>(|_c| {
        Ok(Arc::new(UniqueIdFactory::new()))
    });

    if env.ag_smtp_host.is_empty() {
        container.provide_factory::<dyn NotifierTrait<Id = UniqueId>, _>(|_c| {
            Ok(Arc::new(StdoutNotifier::new()))
        });
    } else {
        let smtp_transport = Arc::new(build_smtp_transport(
            &env.ag_smtp_host,
            env.ag_smtp_port,
            &env.ag_smtp_tls_mode,
            &env.ag_smtp_username,
            &env.ag_smtp_password,
        )?);
        let email_from = env.ag_email_from.clone();

        container.provide_factory::<dyn NotifierTrait<Id = UniqueId>, _>(move |c| {
            let member_repository = c.resolve::<dyn MemberRepositoryTrait<Id = UniqueId>>()?;

            Ok(Arc::new(EmailNotifier::<UniqueId>::new(
                &email_from,
                member_repository,
                smtp_transport.clone(),
            )))
        });
    }

    event_business_provide(container);

    rest_infrastructure_provide::<UniqueId, L>(container);

    user_business_provide::<UniqueId, L>(container);

    let password_min_length = env.ag_password_min_length;
    let password_complexity_enabled = env.ag_password_complexity_enabled;

    user_account_business_provide::<UniqueId, L>(
        container,
        env.ag_product_name.clone(),
        3600 * 24 * 30, // TTL 30 days
        3600,           // TTL 1h
        env.ag_email_confirmation_front_url.clone(),
        env.ag_restore_password_front_url.clone(),
        password_min_length,
        password_complexity_enabled,
    );

    oidc_business_provide::<UniqueId, L>(container);

    user_infrastructure_provide::<UniqueId, L>(container, env.ag_user_database_url.clone(), 10)
        .await?;

    contact_us_business_provide::<UniqueId, L>(container);

    contact_us_infrastructure_provider::<UniqueId, L>(
        container,
        env.ag_contact_us_database_url.clone(),
        10,
        env.ag_contact_us_admin_email.clone(),
        env.ag_email_from.clone(),
        env.ag_smtp_host.clone(),
        env.ag_smtp_port,
        env.ag_smtp_tls_mode.clone(),
        env.ag_smtp_username.clone(),
        env.ag_smtp_password.clone(),
    )
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
        .map_err(|e| DiError::with_context("Can't create reqwest client", e))?;
    let client = Arc::new(client);

    oidc_infrastructure_provider::<L>(
        container,
        env.ag_oidc_database_url.clone(),
        10,
        env.ag_github_client_id.clone(),
        env.ag_github_client_secret.clone(),
        env.ag_google_client_id.clone(),
        env.ag_google_client_secret.clone(),
        env.ag_oidc_redirect_url.clone(),
        client.clone(),
    )
    .await?;

    if env.ag_captcha_enabled {
        if env.ag_captcha_turnstile_secret_key.is_empty() {
            return Err(DiError::from(
                "AG_CAPTCHA_TURNSTILE_SECRET_KEY is required when AG_CAPTCHA_ENABLED=true",
            ));
        }

        captcha_infrastructure_provide_turnstile(
            container,
            SecretString::from(env.ag_captcha_turnstile_secret_key.clone()),
            client,
        );
    } else {
        captcha_business_provide_mock(container);
    }
    captcha_business_provide::<L>(container, env.ag_captcha_enabled);

    let pbkdf2 = Arc::new(Pbkdf2::new());
    let encryptor = pbkdf2.clone();

    container.provide_factory::<dyn Encryptor, _>(move |_c| Ok(encryptor.clone()));
    container.provide_factory::<dyn Validator, _>(move |_c| Ok(pbkdf2.clone()));

    container.provide_factory::<Http1Server<L>, _>(move |c| {
        let logger = c.resolve::<L>()?;

        let router = resolve_router(c)?;
        let response_transformer = c.resolve::<ResponseToJsonTransformer>()?;
        let error_handler = c.resolve::<ErrorHandler<L>>()?;
        let addr = listen
            .parse()
            .map_err(|e| DiError::with_context("Unable to parse socket address", e))?;

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

        let router = resolve_router(c)?;
        let response_transformer = c.resolve::<ResponseToJsonTransformer>()?;
        let error_handler = c.resolve::<ErrorHandler<L>>()?;

        let addr = listen
            .parse()
            .map_err(|e| DiError::with_context("Unable to parse socket address", e))?;

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
