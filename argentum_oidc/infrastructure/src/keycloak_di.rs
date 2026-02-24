use argentum_log_business::LoggerTrait;
use argentum_oidc_business::di::{OidcBusinessDi, OidcBusinessDiBuilder};
use argentum_oidc_rest::ApiDiC;
use argentum_rest_infrastructure::di::RestDiBuilder;
use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::di::UserAccountBusinessDiBuilder;
use argentum_user_business::di::UserBusinessDiBuilder;

use std::sync::{Arc, Mutex};

use crate::{
    keycloak::{
        repository::{MemberRepository, PasswordCredentialRepository},
        service::{
            AdminApi, KeycloakBackendClient, KeycloakEmailVerifier, KeycloakFrontendClient,
            KeycloakMemberAuthenticator,
        },
    },
    rest::handler::ExchangeCodeHandler,
};

pub struct OidcInfrastructureDi<I, L>
where
    I: Id,
    L: LoggerTrait + 'static,
{
    pub business_di: OidcBusinessDi<I, L>,
    pub api_di: ApiDiC<I>,
}

pub struct KeycloakInfrastructureDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    keycloak_base_url: String,
    keycloak_realm: String,
    oidc_redirect_uri: String, // Frontend callback

    keycloak_be_client_id: String,
    keycloak_be_client_secret: String,
    keycloak_fe_client_id: String,

    business_builder: Arc<Mutex<OidcBusinessDiBuilder<UniqueId, L>>>,
    user_business_builder: Arc<Mutex<UserBusinessDiBuilder<UniqueId, L>>>,
    user_account_business_builder: Arc<Mutex<UserAccountBusinessDiBuilder<UniqueId, L>>>,
    rest_builder: Arc<RestDiBuilder<UniqueId, L>>,

    logger: Arc<L>,
    client: Arc<reqwest::Client>,
}

impl<L> KeycloakInfrastructureDiBuilder<L>
where
    L: LoggerTrait + 'static,
{
    pub fn new(
        keycloak_base_url: &str,
        keycloak_realm: &str,
        oidc_redirect_uri: &str, // Frontend callback
        keycloak_be_client_id: &str,
        keycloak_be_client_secret: &str,
        keycloak_fe_client_id: &str,

        user_business_builder: Arc<Mutex<UserBusinessDiBuilder<UniqueId, L>>>,
        user_account_business_builder: Arc<Mutex<UserAccountBusinessDiBuilder<UniqueId, L>>>,
        rest_builder: Arc<RestDiBuilder<UniqueId, L>>,

        id_factory: Arc<dyn IdFactory<Id = UniqueId>>,
        logger: Arc<L>,
        client: Arc<reqwest::Client>,
    ) -> Self {
        Self {
            keycloak_base_url: keycloak_base_url.into(),
            keycloak_realm: keycloak_realm.into(),
            oidc_redirect_uri: oidc_redirect_uri.into(),
            keycloak_be_client_id: keycloak_be_client_id.into(),
            keycloak_be_client_secret: keycloak_be_client_secret.into(),
            keycloak_fe_client_id: keycloak_fe_client_id.into(),

            user_business_builder: user_business_builder.clone(),
            user_account_business_builder: user_account_business_builder.clone(),

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
        let keycloak_be_client = Arc::new(KeycloakBackendClient::new(
            &self.keycloak_base_url.clone(),
            &self.keycloak_realm.clone(),
            &self.keycloak_be_client_id.clone(),
            &self.keycloak_be_client_secret.clone(),
            &self.oidc_redirect_uri.clone(),
            self.client.clone(),
        ));

        let keycloak_fe_client = Arc::new(KeycloakFrontendClient::new(
            &self.keycloak_base_url.clone(),
            &self.keycloak_realm.clone(),
            &self.keycloak_fe_client_id.clone(),
            &self.oidc_redirect_uri.clone(),
            self.client.clone(),
        ));

        self.business_builder
            .lock()
            .unwrap()
            .with_code_exchanger(keycloak_fe_client.clone());

        let admin_api = Arc::new(AdminApi::new(
            &self.keycloak_base_url,
            &self.keycloak_realm.clone(),
            &self.keycloak_be_client_id.clone(),
            &self.keycloak_be_client_secret.clone(),
            self.logger.clone(),
            self.client.clone(),
        ));

        let member_repository = Arc::new(MemberRepository::new(
            admin_api.clone(),
            self.logger.clone(),
        ));

        let password_credential_repository = Arc::new(PasswordCredentialRepository::new(
            admin_api,
            self.logger.clone(),
        ));

        let member_authenticator = Arc::new(KeycloakMemberAuthenticator::new(
            &self.keycloak_base_url,
            &self.keycloak_realm.clone(),
            &self.keycloak_be_client_id.clone(),
            &self.keycloak_be_client_secret.clone(),
            self.client.clone(),
        ));
        let member_confirms_email_uc = Arc::new(KeycloakEmailVerifier::new(
            &self.keycloak_base_url,
            &self.keycloak_realm.clone(),
            self.client.clone(),
        ));

        self.user_business_builder
            .lock()
            .unwrap()
            .with_member_authenticator(member_authenticator)
            .with_member_repository(member_repository);

        self.user_account_business_builder
            .lock()
            .unwrap()
            .with_member_confirms_email_uc(member_confirms_email_uc)
            .with_password_credential_repository(password_credential_repository)
            .with_password_to_token_exchanger(keycloak_be_client);

        Ok(self)
    }

    pub fn build(&mut self) -> Result<OidcInfrastructureDi<UniqueId, L>, String> {
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
