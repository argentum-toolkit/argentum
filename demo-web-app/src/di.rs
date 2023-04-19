use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_notification_business::mock::StdoutNotificator;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_account_business::repository::password_credential_checker::PasswordCredentialChecker;
use argentum_user_account_business::repository::password_credential_writer::PasswordCredentialWriter;
use argentum_user_account_business::use_case::anonymous_registers::AnonymousRegistersUc;
use argentum_user_account_business::use_case::restore_password::anonymous_requests_restore_token::AnonymousRequestsRestoreToken;
use argentum_user_account_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use argentum_user_account_business::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
use argentum_user_account_business::use_case::user_registers_with_password::UserRegistersWithPasswordUc;
use argentum_user_account_infrastructure::token::StringTokenGenerator;
use argentum_user_account_business::use_case::restore_password::anonymous_with_token_changes_password::AnonymousWithTokenChangesPassword;
use argentum_standard_infrastructure::db_diesel::connection::pg::ConnectionPoolManager;
use argentum_user_account_infrastructure::db_diesel::repository::password_credential_repository::PasswordCredentialRepository;
use argentum_user_account_infrastructure::db_diesel::repository::session_repository::SessionRepository;
use argentum_user_account_infrastructure::db_diesel::repository::restore_password_token_repository::RestorePasswordTokenRepository;
use argentum_user_infrastructure::db_diesel::repository::anonymous_binding_repository::AnonymousBindingRepository;
use argentum_user_infrastructure::db_diesel::repository::anonymous_user_repository::AnonymousUserRepository;
use argentum_user_infrastructure::db_diesel::repository::authenticated_user_repository::AuthenticatedUserRepository;

use std::sync::Arc;

pub struct DiC {
    // Public services
    pub id_factory: Arc<UniqueIdFactory>,
    pub logger: Arc<dyn LoggerTrait>,
    pub anonymous_registers_uc: Arc<AnonymousRegistersUc>,
    pub user_registers_with_password_uc: Arc<UserRegistersWithPasswordUc>,
    pub user_logins_with_password_uc: Arc<UserLoginsWithPasswordUc>,
    pub user_authenticates_with_token: Arc<UserAuthenticatesWithTokenUc>,
    pub anonymous_requests_restore_token_uc: Arc<AnonymousRequestsRestoreToken>,
    pub anonymous_with_token_changes_password_uc: Arc<AnonymousWithTokenChangesPassword>,
}

impl DiC {
    pub fn new(
        id_factory: Arc<UniqueIdFactory>,
        logger: Arc<dyn LoggerTrait>,
        anonymous_registers_uc: Arc<AnonymousRegistersUc>,
        user_registers_with_password_uc: Arc<UserRegistersWithPasswordUc>,
        user_logins_with_password_uc: Arc<UserLoginsWithPasswordUc>,
        user_authenticates_with_token: Arc<UserAuthenticatesWithTokenUc>,
        anonymous_requests_restore_token_uc: Arc<AnonymousRequestsRestoreToken>,
        anonymous_with_token_changes_password_uc: Arc<AnonymousWithTokenChangesPassword>,
    ) -> DiC {
        DiC {
            id_factory,
            logger,
            anonymous_registers_uc,
            user_registers_with_password_uc,
            user_logins_with_password_uc,
            user_authenticates_with_token,
            anonymous_requests_restore_token_uc,
            anonymous_with_token_changes_password_uc,
        }
    }
}

pub fn di_factory() -> DiC {
    let user_account_pg_connection_pool_manager =
        Arc::new(ConnectionPoolManager::new("AG_USER_ACCOUNT_DATABASE_URL"));

    let user_pg_connection_pool_manager =
        Arc::new(ConnectionPoolManager::new("AG_USER_DATABASE_URL"));

    let unique_id_factory = Arc::new(UniqueIdFactory::new());

    let anonymous_user_repository = Arc::new(AnonymousUserRepository::new(
        user_pg_connection_pool_manager.clone(),
        unique_id_factory.clone(),
    ));

    let session_repository = Arc::new(SessionRepository::new(
        user_account_pg_connection_pool_manager.clone(),
        unique_id_factory.clone(),
    ));
    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let token_generator = Arc::new(StringTokenGenerator::new());

    let anonymous_registers_uc = Arc::new(AnonymousRegistersUc::new(
        unique_id_factory.clone(),
        anonymous_user_repository.clone(),
        session_repository.clone(),
        token_generator.clone(),
    ));

    let authenticated_user_repository = Arc::new(AuthenticatedUserRepository::new(
        user_pg_connection_pool_manager.clone(),
        unique_id_factory.clone(),
    ));
    let password_credential_repository = Arc::new(PasswordCredentialRepository::new(
        user_account_pg_connection_pool_manager.clone(),
        unique_id_factory.clone(),
    ));

    let password_credential_writer = Arc::new(PasswordCredentialWriter::new(
        password_credential_repository.clone(),
    ));

    let pbkdf2_password_encryptor = Arc::new(Pbkdf2::new());

    let user_registers_with_password_uc = Arc::new(UserRegistersWithPasswordUc::new(
        authenticated_user_repository.clone(),
        password_credential_writer.clone(),
        pbkdf2_password_encryptor.clone(),
    ));

    let password_credential_checker = Arc::new(PasswordCredentialChecker::new(
        password_credential_repository,
        pbkdf2_password_encryptor.clone(),
    ));

    let anonymous_binding_repository = Arc::new(AnonymousBindingRepository::new(
        user_pg_connection_pool_manager,
        unique_id_factory.clone(),
    ));

    let user_logins_with_password_uc = Arc::new(UserLoginsWithPasswordUc::new(
        authenticated_user_repository.clone(),
        anonymous_binding_repository,
        session_repository.clone(),
        password_credential_checker,
        unique_id_factory.clone(),
        token_generator.clone(),
        logger.clone(),
    ));

    let user_authenticates_with_token_uc = Arc::new(UserAuthenticatesWithTokenUc::new(
        authenticated_user_repository.clone(),
        anonymous_user_repository,
        session_repository,
    ));

    let notificator = Arc::new(StdoutNotificator::new());

    let restore_password_token_repository = Arc::new(RestorePasswordTokenRepository::new(
        user_account_pg_connection_pool_manager,
        unique_id_factory.clone(),
    ));
    // let restore_password_token_repository = Arc::new(RestorePasswordTokenRepositoryMock::new());

    let anonymous_requests_restore_token_uc = Arc::new(AnonymousRequestsRestoreToken::new(
        "Argentum ToolKit demo web application".to_string(),
        "http://localhost:8080/change-password/".to_string(),
        unique_id_factory.clone(),
        authenticated_user_repository.clone(),
        restore_password_token_repository.clone(),
        token_generator,
        notificator,
        logger.clone(),
    ));

    let anonymous_with_token_changes_password_uc =
        Arc::new(AnonymousWithTokenChangesPassword::new(
            authenticated_user_repository,
            restore_password_token_repository,
            pbkdf2_password_encryptor,
            password_credential_writer,
            3600, // 1h
            logger.clone(),
        ));

    DiC::new(
        unique_id_factory,
        logger,
        anonymous_registers_uc,
        user_registers_with_password_uc,
        user_logins_with_password_uc,
        user_authenticates_with_token_uc,
        anonymous_requests_restore_token_uc,
        anonymous_with_token_changes_password_uc,
    )
}
