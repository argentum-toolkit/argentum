use crate::app::App;

use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_log_business::{DefaultLogger, Level};
use argentum_log_infrastructure::stdout::PrettyWriter;
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

pub fn init() -> Result<(), String> {
    let anonymous_user_repository = AnonymousUserRepositoryMock::new();
    let session_repository = SessionRepositoryMock::new();
    let unique_id_factory = UniqueIdFactory::new();
    let log_writer = PrettyWriter::new();
    let logger = DefaultLogger::new(Level::Trace, &log_writer);

    let token_generator = StringTokenGenerator::new();

    let anonymous_registers_uc = AnonymousRegistersUc::new(
        &unique_id_factory,
        &anonymous_user_repository,
        &session_repository,
        &token_generator,
    );

    let authenticated_user_repository = AuthenticatedUserRepositoryMock::new();
    let password_credential_repository = PasswordCredentialRepositoryMock::new();

    let password_credential_writer = PasswordCredentialWriter::new(&password_credential_repository);

    let pbkdf2_password = Pbkdf2::new();

    let user_registers_uc = UserRegistersWithPasswordUc::new(
        &authenticated_user_repository,
        &password_credential_writer,
        &pbkdf2_password,
    );

    let password_credential_checker =
        PasswordCredentialChecker::new(&password_credential_repository, &pbkdf2_password);

    let anonymous_binding_repository = AnonymousBindingRepositoryMock::new();
    let user_logins_with_password_uc = UserLoginsWithPasswordUc::new(
        &authenticated_user_repository,
        &anonymous_binding_repository,
        &session_repository,
        &password_credential_checker,
        &unique_id_factory,
        &token_generator,
        &logger,
    );

    let user_authenticates_with_token_uc = UserAuthenticatesWithTokenUc::new(
        &authenticated_user_repository,
        &anonymous_user_repository,
        &session_repository,
    );

    let app = App::new(
        &unique_id_factory,
        &anonymous_registers_uc,
        &user_logins_with_password_uc,
        &user_registers_uc,
        &user_authenticates_with_token_uc,
        &logger,
    );

    app.run()
}
