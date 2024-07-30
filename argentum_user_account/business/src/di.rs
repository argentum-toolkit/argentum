use crate::mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock;
use crate::mock::repository::restore_password_token_repository_mock::RestorePasswordTokenRepositoryMock;
use crate::mock::token::TokenGeneratorMock;
use crate::repository::password_credential_checker::PasswordCredentialChecker;
use crate::repository::password_credential_repository::PasswordCredentialRepositoryTrait;
use crate::repository::password_credential_writer::PasswordCredentialWriter;
use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
use crate::token::GeneratorTrait;
use crate::use_case::anonymous_registers::AnonymousRegistersUc;
use crate::use_case::restore_password::anonymous_requests_restore_token::AnonymousRequestsRestoreTokenUc;
use crate::use_case::restore_password::anonymous_with_token_changes_password::AnonymousWithTokenChangesPasswordUc;
use crate::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
use crate::use_case::user_registers_with_password::UserRegistersWithPasswordUc;
use argentum_encryption_business::password::{Encryptor, Validator};
use argentum_log_business::LoggerTrait;
use argentum_notification_business::NotificatorTrait;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_user_business::mock::repository::anonymous_binding_repository_mock::AnonymousBindingRepositoryMock;
use argentum_user_business::mock::repository::anonymous_user_repository_mock::AnonymousUserRepositoryMock;
use argentum_user_business::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
use argentum_user_business::mock::repository::session_repository_mock::SessionRepositoryMock;
use argentum_user_business::repository::anonymous_binding_repository::AnonymousBindingRepositoryTrait;
use argentum_user_business::repository::session_repository::SessionRepositoryTrait;
use argentum_user_business::repository::user_repository::{
    AnonymousUserRepositoryTrait, AuthenticatedUserRepositoryTrait,
};
use argentum_user_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use std::sync::Arc;

pub struct BusinessDiC {
    // Public services
    pub anonymous_registers_uc: Arc<AnonymousRegistersUc>,
    pub user_registers_with_password_uc: Arc<UserRegistersWithPasswordUc>,
    pub user_logins_with_password_uc: Arc<UserLoginsWithPasswordUc>,
    pub user_authenticates_with_token_uc: Arc<UserAuthenticatesWithTokenUc>,
    pub anonymous_with_token_changes_password_uc: Arc<AnonymousWithTokenChangesPasswordUc>,
    pub anonymous_requests_restore_token_uc: Arc<AnonymousRequestsRestoreTokenUc>,
}

pub struct UserAccountBusinessDiCBuilder {
    id_factory: Arc<dyn IdFactory>,
    encryptor: Arc<dyn Encryptor>,
    validator: Arc<dyn Validator>,
    logger: Arc<dyn LoggerTrait>,
    notificator: Arc<dyn NotificatorTrait>,

    anonymous_binding_repository: Option<Arc<dyn AnonymousBindingRepositoryTrait>>,
    anonymous_user_repository: Option<Arc<dyn AnonymousUserRepositoryTrait>>,
    authenticated_user_repository: Option<Arc<dyn AuthenticatedUserRepositoryTrait>>,
    session_repository: Option<Arc<dyn SessionRepositoryTrait>>,
    password_credential_repository: Option<Arc<dyn PasswordCredentialRepositoryTrait>>,
    restore_password_token_repository: Option<Arc<dyn RestorePasswordTokenRepositoryTrait>>,
    token_generator: Option<Arc<dyn GeneratorTrait>>,

    restore_password_token_ttl: u32,
    product_name: String,
    restore_password_front_url: String,
}

impl UserAccountBusinessDiCBuilder {
    pub fn new(
        id_factory: Arc<dyn IdFactory>,
        encryptor: Arc<dyn Encryptor>,
        validator: Arc<dyn Validator>,
        logger: Arc<dyn LoggerTrait>,
        notificator: Arc<dyn NotificatorTrait>,
    ) -> Self {
        Self {
            id_factory,
            encryptor,
            validator,
            logger,
            notificator,
            anonymous_binding_repository: None,
            anonymous_user_repository: None,
            authenticated_user_repository: None,
            session_repository: None,
            password_credential_repository: None,
            restore_password_token_repository: None,
            token_generator: None,
            restore_password_token_ttl: 1,
            product_name: "".to_string(),
            restore_password_front_url: "".to_string(),
        }
    }

    pub fn mock(&mut self) -> &mut Self {
        self.anonymous_binding_repository = Some(Arc::new(AnonymousBindingRepositoryMock::new()));
        self.anonymous_user_repository = Some(Arc::new(AnonymousUserRepositoryMock::new()));
        self.authenticated_user_repository = Some(Arc::new(AuthenticatedUserRepositoryMock::new()));
        self.session_repository = Some(Arc::new(SessionRepositoryMock::new()));

        self.password_credential_repository =
            Some(Arc::new(PasswordCredentialRepositoryMock::new()));
        self.restore_password_token_repository =
            Some(Arc::new(RestorePasswordTokenRepositoryMock::new()));

        self.token_generator = Some(Arc::new(TokenGeneratorMock::new()));

        self
    }

    pub fn config(
        &mut self,
        product_name: String,
        restore_password_token_ttl: u32,
        restore_password_front_url: String,
    ) -> &mut Self {
        self.product_name = product_name;
        self.restore_password_token_ttl = restore_password_token_ttl;
        self.restore_password_front_url = restore_password_front_url;

        self
    }

    pub fn services(
        &mut self,
        anonymous_binding_repository: Arc<dyn AnonymousBindingRepositoryTrait>,
        anonymous_user_repository: Arc<dyn AnonymousUserRepositoryTrait>,
        authenticated_user_repository: Arc<dyn AuthenticatedUserRepositoryTrait>,
        session_repository: Arc<dyn SessionRepositoryTrait>,
        password_credential_repository: Arc<dyn PasswordCredentialRepositoryTrait>,
        restore_password_token_repository: Arc<dyn RestorePasswordTokenRepositoryTrait>,
        token_generator: Arc<dyn GeneratorTrait>,
    ) -> &mut Self {
        self.anonymous_binding_repository = Some(anonymous_binding_repository);
        self.anonymous_user_repository = Some(anonymous_user_repository);
        self.authenticated_user_repository = Some(authenticated_user_repository);
        self.session_repository = Some(session_repository);
        self.password_credential_repository = Some(password_credential_repository);
        self.restore_password_token_repository = Some(restore_password_token_repository);
        self.token_generator = Some(token_generator);

        self
    }

    pub fn build(&self) -> BusinessDiC {
        let anonymous_registers_uc = Arc::new(AnonymousRegistersUc::new(
            self.id_factory.clone(),
            self.anonymous_user_repository.clone().unwrap(),
            self.session_repository.clone().unwrap(),
            self.token_generator.clone().unwrap(),
        ));

        let password_credential_writer = Arc::new(PasswordCredentialWriter::new(
            self.password_credential_repository.clone().unwrap(),
        ));

        let user_registers_with_password_uc = Arc::new(UserRegistersWithPasswordUc::new(
            self.authenticated_user_repository.clone().unwrap(),
            password_credential_writer.clone(),
            self.encryptor.clone(),
        ));

        let password_credential_checker = Arc::new(PasswordCredentialChecker::new(
            self.password_credential_repository.clone().unwrap(),
            self.validator.clone(),
        ));

        let user_logins_with_password_uc = Arc::new(UserLoginsWithPasswordUc::new(
            self.authenticated_user_repository.clone().unwrap(),
            self.anonymous_binding_repository.clone().unwrap(),
            self.session_repository.clone().unwrap(),
            password_credential_checker,
            self.id_factory.clone(),
            self.token_generator.clone().unwrap(),
            self.logger.clone(),
        ));

        let user_authenticates_with_token_uc = Arc::new(UserAuthenticatesWithTokenUc::new(
            self.authenticated_user_repository.clone().unwrap(),
            self.anonymous_user_repository.clone().unwrap(),
            self.session_repository.clone().unwrap(),
        ));

        let anonymous_with_token_changes_password_uc =
            Arc::new(AnonymousWithTokenChangesPasswordUc::new(
                self.authenticated_user_repository.clone().unwrap(),
                self.restore_password_token_repository.clone().unwrap(),
                self.encryptor.clone(),
                password_credential_writer,
                self.restore_password_token_ttl,
                self.logger.clone(),
            ));

        let anonymous_requests_restore_token_uc = Arc::new(AnonymousRequestsRestoreTokenUc::new(
            self.product_name.clone(),
            self.restore_password_front_url.clone(),
            self.id_factory.clone(),
            self.authenticated_user_repository.clone().unwrap(),
            self.restore_password_token_repository.clone().unwrap(),
            self.token_generator.clone().unwrap(),
            self.notificator.clone(),
            self.logger.clone(),
        ));

        BusinessDiC {
            anonymous_registers_uc,
            user_registers_with_password_uc,
            user_logins_with_password_uc,
            user_authenticates_with_token_uc,
            anonymous_with_token_changes_password_uc,
            anonymous_requests_restore_token_uc,
        }
    }
}
