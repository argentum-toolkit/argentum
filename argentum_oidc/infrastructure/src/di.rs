use argentum_log_business::LoggerTrait;
use argentum_oidc_business::di::{OidcBusinessDiC, OidcBusinessDiCBuilder};
use argentum_oidc_rest::server::handler::ExchangeCodeTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::{
    repository::password_credential_repository::PasswordCredentialRepositoryTrait,
    service::PasswordToTokenExchangerTrait,
    use_case::member_confirms_email::MemberConfirmsEmailUcTrait,
};
use argentum_user_business::service::MemberAuthenticatorTrait;

use std::sync::Arc;

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

pub struct OidcInfrastructureDiC<I, L>
where
    I: Id,
    L: LoggerTrait + 'static,
{
    pub business_dic: OidcBusinessDiC<I>,

    // Public services
    pub exchange_code_handler: Arc<dyn ExchangeCodeTrait<Id = I>>,
    pub member_repository: Arc<MemberRepository<L>>,
    pub password_credential_repository: Arc<dyn PasswordCredentialRepositoryTrait<Id = I>>,
    pub password_to_token_exchanger: Arc<dyn PasswordToTokenExchangerTrait<Id = I>>,
    pub member_authenticator: Arc<dyn MemberAuthenticatorTrait<Id = I>>,
    pub member_confirms_email_uc: Arc<dyn MemberConfirmsEmailUcTrait<Id = I>>,
}

pub struct KeycloakInfrastructureDiCBuilder<L>
where
    L: LoggerTrait + 'static,
{
    keycloak_base_url: String,
    keycloak_realm: String,
    oidc_redirect_uri: String, // Frontend callback

    keycloak_be_client_id: String,
    keycloak_be_client_secret: String,
    keycloak_fe_client_id: String,

    business_builder: OidcBusinessDiCBuilder<UniqueId>,

    logger: Arc<L>,
    client: Arc<reqwest::Client>,
}

impl<L> KeycloakInfrastructureDiCBuilder<L>
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

            business_builder: OidcBusinessDiCBuilder::new(),
            logger,
            client,
        }
    }

    pub fn build(&mut self) -> Result<OidcInfrastructureDiC<UniqueId, L>, String> {
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

        let business_dic = self
            .business_builder
            .with_auth_code_exchanger(keycloak_fe_client.clone())
            .build()?;

        let exchange_code_handler = Arc::new(ExchangeCodeHandler::new(
            business_dic.user_exchanges_code_to_token_uc.clone(),
        ));

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

        Ok(OidcInfrastructureDiC {
            business_dic,
            exchange_code_handler,
            member_repository,
            password_credential_repository,
            password_to_token_exchanger: keycloak_be_client,
            member_authenticator,
            member_confirms_email_uc,
        })
    }
}
