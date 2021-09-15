use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_account_business::mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock;
use argentum_user_account_business::mock::repository::session_repository_mock::SessionRepositoryMock;
use argentum_user_account_business::repository::password_credential_checker::PasswordCredentialChecker;
use argentum_user_account_business::repository::password_credential_writer::PasswordCredentialWriter;
use argentum_user_account_business::use_case::anonymous_registers::AnonymousRegistersUc;
use argentum_user_account_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use argentum_user_account_business::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
use argentum_user_account_business::use_case::user_registers_with_password::UserRegistersWithPasswordUc;
use argentum_user_account_infrastructure::token::StringTokenGenerator;
use argentum_user_business::mock::repository::anonymous_binding_repository_mock::AnonymousBindingRepositoryMock;
use argentum_user_business::mock::repository::anonymous_user_repository_mock::AnonymousUserRepositoryMock;
use argentum_user_business::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
use std::sync::Arc;

pub struct DiC {
    // Public services
    pub id_factory: Arc<dyn IdFactory>,
    pub anonymous_registers_uc: Arc<AnonymousRegistersUc>,
    pub user_logins_with_pw: Arc<UserLoginsWithPasswordUc>,
    pub user_registers_with_pw: Arc<UserRegistersWithPasswordUc>,
    pub user_authenticates_with_token: Arc<UserAuthenticatesWithTokenUc>,
    pub logger: Arc<dyn LoggerTrait>,
}

impl DiC {
    pub fn new(
        id_factory: Arc<dyn IdFactory>,
        anonymous_registers_uc: Arc<AnonymousRegistersUc>,
        user_logins_with_pw: Arc<UserLoginsWithPasswordUc>,
        user_registers_with_pw: Arc<UserRegistersWithPasswordUc>,
        user_authenticates_with_token: Arc<UserAuthenticatesWithTokenUc>,
        logger: Arc<dyn LoggerTrait>,
    ) -> DiC {
        DiC {
            id_factory,
            anonymous_registers_uc,
            user_logins_with_pw,
            user_registers_with_pw,
            user_authenticates_with_token,
            logger,
        }
    }
}

pub fn di_factory() -> DiC {
    let anonymous_user_repository = Arc::new(AnonymousUserRepositoryMock::new());
    let session_repository = Arc::new(SessionRepositoryMock::new());
    let unique_id_factory = Arc::new(UniqueIdFactory::new());
    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let token_generator = Arc::new(StringTokenGenerator::new());

    let anonymous_registers_uc = Arc::new(AnonymousRegistersUc::new(
        unique_id_factory.clone(),
        anonymous_user_repository.clone(),
        session_repository.clone(),
        token_generator.clone(),
    ));

    let authenticated_user_repository = Arc::new(AuthenticatedUserRepositoryMock::new());
    let password_credential_repository = Arc::new(PasswordCredentialRepositoryMock::new());

    let password_credential_writer = Arc::new(PasswordCredentialWriter::new(
        password_credential_repository.clone(),
    ));

    let pbkdf2_password = Arc::new(Pbkdf2::new());

    let user_registers_with_password_uc = Arc::new(UserRegistersWithPasswordUc::new(
        authenticated_user_repository.clone(),
        password_credential_writer,
        pbkdf2_password.clone(),
    ));

    let password_credential_checker = Arc::new(PasswordCredentialChecker::new(
        password_credential_repository,
        pbkdf2_password,
    ));

    let anonymous_binding_repository = Arc::new(AnonymousBindingRepositoryMock::new());
    let user_logins_with_password_uc = Arc::new(UserLoginsWithPasswordUc::new(
        authenticated_user_repository.clone(),
        anonymous_binding_repository,
        session_repository.clone(),
        password_credential_checker,
        unique_id_factory.clone(),
        token_generator,
        logger.clone(),
    ));

    let user_authenticates_with_token_uc = Arc::new(UserAuthenticatesWithTokenUc::new(
        authenticated_user_repository,
        anonymous_user_repository,
        session_repository,
    ));

    DiC::new(
        unique_id_factory,
        anonymous_registers_uc,
        user_logins_with_password_uc,
        user_registers_with_password_uc,
        user_authenticates_with_token_uc,
        logger,
    )
}
