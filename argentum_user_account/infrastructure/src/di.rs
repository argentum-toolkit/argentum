use crate::api::server::handler::{
    AnonymousRegistersTrait, AnonymousWithTokenChangesPasswordTrait, UserLoginsWithPasswordTrait,
    UserRegistersWithPasswordTrait,
};
use crate::db_diesel::repository::password_credential_repository::PasswordCredentialRepository;
use crate::db_diesel::repository::restore_password_token_repository::RestorePasswordTokenRepository;
use crate::db_diesel::repository::session_repository::SessionRepository;
use crate::rest::handler::{
    AnonymousRegistersHandler, AnonymousRequestsRestoreTokenHandler,
    AnonymousWithTokenChangesPasswordHandler, UserLoginsWithPasswordHandler,
    UserRegistersWithPasswordHandler,
};
use crate::rest::transformer::{
    DtoToAnonymousRequestsRestoreTokenParams, DtoToAnonymousWithTokenChangesPasswordParams,
    DtoToUserLoginsWithPasswordParams, DtoToUserRegistersWithPasswordParams,
};
use crate::token::StringTokenGenerator;
use argentum_encryption_business::password::{Encryptor, Validator};
use argentum_log_business::LoggerTrait;
use argentum_notification_business::NotificatorTrait;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db_diesel::connection::pg::ConnectionPoolManager;
use argentum_user_account_business::di::BusinessDiC;
use argentum_user_account_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use argentum_user_business::repository::anonymous_binding_repository::AnonymousBindingRepositoryTrait;
use argentum_user_business::repository::user_repository::{
    AnonymousUserRepositoryTrait, AuthenticatedUserRepositoryTrait,
};
use std::sync::Arc;

pub struct UserAccountInfrastructureDiC {
    // Public services
    pub anonymous_registers_handler: Arc<dyn AnonymousRegistersTrait>,
    pub anonymous_requests_restore_token: Arc<AnonymousRequestsRestoreTokenHandler>,
    pub anonymous_with_token_changes_password: Arc<dyn AnonymousWithTokenChangesPasswordTrait>,
    pub user_registers_with_password_handler: Arc<dyn UserRegistersWithPasswordTrait>,
    pub user_logins_with_password_handler: Arc<dyn UserLoginsWithPasswordTrait>,
    pub user_authenticates_with_token_uc: Arc<UserAuthenticatesWithTokenUc>,
}

impl UserAccountInfrastructureDiC {
    pub fn new(
        anonymous_user_repository: Arc<dyn AnonymousUserRepositoryTrait>,
        authenticated_user_repository: Arc<dyn AuthenticatedUserRepositoryTrait>,
        anonymous_binding_repository: Arc<dyn AnonymousBindingRepositoryTrait>,
        id_factory: Arc<UniqueIdFactory>,
        connection: Arc<ConnectionPoolManager>,
        encryptor: Arc<dyn Encryptor>,
        validator: Arc<dyn Validator>,
        logger: Arc<dyn LoggerTrait>,
        notificator: Arc<dyn NotificatorTrait>,
        restore_password_token_ttl: u32,
        product_name: String,
        restore_password_front_url: String,
    ) -> Self {
        let session_repository = Arc::new(SessionRepository::new(
            connection.clone(),
            id_factory.clone(),
        ));
        let token_generator = Arc::new(StringTokenGenerator::new());

        let password_credential_repository = Arc::new(PasswordCredentialRepository::new(
            connection.clone(),
            id_factory.clone(),
        ));

        let restore_password_token_repository = Arc::new(RestorePasswordTokenRepository::new(
            connection,
            id_factory.clone(),
        ));

        let bdi = BusinessDiC::new(
            anonymous_user_repository,
            authenticated_user_repository,
            anonymous_binding_repository.clone(),
            session_repository,
            id_factory.clone(),
            token_generator,
            password_credential_repository,
            restore_password_token_repository,
            encryptor.clone(),
            validator.clone(),
            logger.clone(),
            notificator,
            restore_password_token_ttl,
            product_name,
            restore_password_front_url,
        );

        let anonymous_registers_handler = Arc::new(AnonymousRegistersHandler::new(
            bdi.anonymous_registers_uc,
            id_factory.clone(),
        ));

        let dto_to_user_registers_with_password_params =
            Arc::new(DtoToUserRegistersWithPasswordParams::new());

        let user_registers_with_password_handler = Arc::new(UserRegistersWithPasswordHandler::new(
            bdi.user_registers_with_password_uc,
            id_factory.clone(),
            dto_to_user_registers_with_password_params,
        ));

        let dto_to_user_logins_with_password_params =
            Arc::new(DtoToUserLoginsWithPasswordParams::new());

        let user_logins_with_password_handler = Arc::new(UserLoginsWithPasswordHandler::new(
            bdi.user_logins_with_password_uc,
            id_factory,
            dto_to_user_logins_with_password_params,
        ));

        let dto_to_anonymous_with_token_changes_password_params =
            Arc::new(DtoToAnonymousWithTokenChangesPasswordParams::new());

        let anonymous_with_token_changes_password =
            Arc::new(AnonymousWithTokenChangesPasswordHandler::new(
                bdi.anonymous_with_token_changes_password_uc,
                dto_to_anonymous_with_token_changes_password_params,
            ));

        let dto_to_anonymous_requests_restore_token_params =
            Arc::new(DtoToAnonymousRequestsRestoreTokenParams::new());

        let anonymous_requests_restore_token = Arc::new(AnonymousRequestsRestoreTokenHandler::new(
            bdi.anonymous_requests_restore_token_uc,
            logger.clone(),
            dto_to_anonymous_requests_restore_token_params,
        ));

        UserAccountInfrastructureDiC {
            anonymous_registers_handler,
            user_registers_with_password_handler,
            user_logins_with_password_handler,
            user_authenticates_with_token_uc: bdi.user_authenticates_with_token_uc,
            anonymous_with_token_changes_password,
            anonymous_requests_restore_token,
        }
    }
}
