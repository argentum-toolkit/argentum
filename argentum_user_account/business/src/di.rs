use crate::repository::password_credential_checker::PasswordCredentialChecker;
use crate::repository::password_credential_repository::PasswordCredentialRepositoryTrait;
use crate::repository::password_credential_writer::PasswordCredentialWriter;
use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
use crate::repository::session_repository::SessionRepositoryTrait;
use crate::use_case::anonymous_registers::AnonymousRegistersUc;
use crate::use_case::restore_password::anonymous_requests_restore_token::AnonymousRequestsRestoreTokenUc;
use crate::use_case::restore_password::anonymous_with_token_changes_password::AnonymousWithTokenChangesPasswordUc;
use crate::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use crate::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
use crate::use_case::user_registers_with_password::UserRegistersWithPasswordUc;
use argentum_encryption_business::password::{Encryptor, Validator};
use argentum_log_business::LoggerTrait;
use argentum_notification_business::NotificatorTrait;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_user_business::repository::anonymous_binding_repository::AnonymousBindingRepositoryTrait;
use argentum_user_business::repository::user_repository::{
    AnonymousUserRepositoryTrait, AuthenticatedUserRepositoryTrait,
};
use argentum_user_business::token::GeneratorTrait;
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

impl BusinessDiC {
    pub fn new(
        anonymous_user_repository: Arc<dyn AnonymousUserRepositoryTrait>,
        authenticated_user_repository: Arc<dyn AuthenticatedUserRepositoryTrait>,
        anonymous_binding_repository: Arc<dyn AnonymousBindingRepositoryTrait>,
        session_repository: Arc<dyn SessionRepositoryTrait>,
        id_factory: Arc<dyn IdFactory>,
        token_generator: Arc<dyn GeneratorTrait>,
        password_credential_repository: Arc<dyn PasswordCredentialRepositoryTrait>,
        restore_password_token_repository: Arc<dyn RestorePasswordTokenRepositoryTrait>,
        encryptor: Arc<dyn Encryptor>,
        validator: Arc<dyn Validator>,
        logger: Arc<dyn LoggerTrait>,
        notificator: Arc<dyn NotificatorTrait>,
        restore_password_token_ttl: u32,
        product_name: String,
        restore_password_front_url: String,
    ) -> Self {
        let anonymous_registers_uc = Arc::new(AnonymousRegistersUc::new(
            id_factory.clone(),
            anonymous_user_repository.clone(),
            session_repository.clone(),
            token_generator.clone(),
        ));

        let password_credential_writer = Arc::new(PasswordCredentialWriter::new(
            password_credential_repository.clone(),
        ));

        let user_registers_with_password_uc = Arc::new(UserRegistersWithPasswordUc::new(
            authenticated_user_repository.clone(),
            password_credential_writer.clone(),
            encryptor.clone(),
        ));

        let password_credential_checker = Arc::new(PasswordCredentialChecker::new(
            password_credential_repository.clone(),
            validator.clone(),
        ));

        let user_logins_with_password_uc = Arc::new(UserLoginsWithPasswordUc::new(
            authenticated_user_repository.clone(),
            anonymous_binding_repository,
            session_repository.clone(),
            password_credential_checker,
            id_factory.clone(),
            token_generator.clone(),
            logger.clone(),
        ));

        let user_authenticates_with_token_uc = Arc::new(UserAuthenticatesWithTokenUc::new(
            authenticated_user_repository.clone(),
            anonymous_user_repository,
            session_repository,
        ));

        let anonymous_with_token_changes_password_uc =
            Arc::new(AnonymousWithTokenChangesPasswordUc::new(
                authenticated_user_repository.clone(),
                restore_password_token_repository.clone(),
                encryptor,
                password_credential_writer,
                restore_password_token_ttl,
                logger.clone(),
            ));

        let anonymous_requests_restore_token_uc = Arc::new(AnonymousRequestsRestoreTokenUc::new(
            product_name,
            restore_password_front_url,
            id_factory,
            authenticated_user_repository,
            restore_password_token_repository,
            token_generator,
            notificator,
            logger,
        ));

        Self {
            anonymous_registers_uc,
            user_registers_with_password_uc,
            user_logins_with_password_uc,
            user_authenticates_with_token_uc,
            anonymous_with_token_changes_password_uc,
            anonymous_requests_restore_token_uc,
        }
    }
}
