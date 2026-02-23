use argentum_encryption_business::mock::password::EncryptorMock;
use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_notification_business::mock::StdoutNotifier;
use argentum_standard_business::data_type::id::{IdFactory, UIntId};
use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
use argentum_user_account_business::di::UserAccountBusinessDiBuilder;
use argentum_user_account_business::mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock;
use argentum_user_account_business::mock::repository::restore_password_token_repository_mock::RestorePasswordTokenRepositoryMock;
use argentum_user_account_business::mock::token::TokenGeneratorMock;
use argentum_user_account_business::use_case::anonymous_registers::AnonymousRegistersUc;
use argentum_user_account_business::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
use argentum_user_account_business::use_case::user_registers_with_password::UserRegistersWithPasswordUc;
use argentum_user_business::di::UserBusinessDiBuilder;
use argentum_user_business::mock::repository::anonymous_binding_repository_mock::AnonymousBindingRepositoryMock;
use argentum_user_business::mock::repository::anonymous_user_repository_mock::AnonymousUserRepositoryMock;
use argentum_user_business::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
use argentum_user_business::mock::repository::session_repository_mock::SessionRepositoryMock;
use argentum_user_business::mock::service::{MemberAuthenticatorMock, TokenRouterMock};
use argentum_user_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use std::sync::{Arc, Mutex};

pub struct DiC<L>
where
    L: LoggerTrait,
{
    // Public services
    pub id_factory: Arc<dyn IdFactory<Id = UIntId>>,
    pub anonymous_registers_uc: Arc<AnonymousRegistersUc<UIntId>>,
    pub user_logins_with_pw: Arc<UserLoginsWithPasswordUc<UIntId, L>>,
    pub user_registers_with_pw: Arc<UserRegistersWithPasswordUc<UIntId>>,
    pub user_authenticates_with_token: Arc<UserAuthenticatesWithTokenUc<UIntId>>,
    pub logger: Arc<L>,
}

impl<L> DiC<L>
where
    L: LoggerTrait,
{
    pub fn new(
        id_factory: Arc<dyn IdFactory<Id = UIntId>>,
        anonymous_registers_uc: Arc<AnonymousRegistersUc<UIntId>>,
        user_logins_with_pw: Arc<UserLoginsWithPasswordUc<UIntId, L>>,
        user_registers_with_pw: Arc<UserRegistersWithPasswordUc<UIntId>>,
        user_authenticates_with_token: Arc<UserAuthenticatesWithTokenUc<UIntId>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            id_factory,
            anonymous_registers_uc,
            user_logins_with_pw,
            user_registers_with_pw,
            user_authenticates_with_token,
            logger,
        }
    }
}

pub fn di_factory() -> Result<DiC<DefaultLogger<PrettyWriter>>, String> {
    let id_factory = Arc::new(IdFactoryMock::new());
    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let notificator = Arc::new(StdoutNotifier::new());

    let anonymous_binding_repository = Arc::new(AnonymousBindingRepositoryMock::<UIntId>::new());
    let anonymous_user_repository = Arc::new(AnonymousUserRepositoryMock::<UIntId>::new());
    let authenticated_user_repository = Arc::new(AuthenticatedUserRepositoryMock::<UIntId>::new());
    let session_repository = Arc::new(SessionRepositoryMock::<UIntId>::new());

    let encryptor = Arc::new(EncryptorMock::new());
    let password_credential_repository =
        Arc::new(PasswordCredentialRepositoryMock::<UIntId>::new(encryptor));
    let restore_password_token_repository =
        Arc::new(RestorePasswordTokenRepositoryMock::<UIntId>::new());
    let guest_token_generator = Arc::new(TokenGeneratorMock::<UIntId>::new());
    let member_token_generator = Arc::new(TokenGeneratorMock::<UIntId>::new());
    let token_router = Arc::new(TokenRouterMock::new());
    let member_authenticator = Arc::new(MemberAuthenticatorMock::new());

    let user_business_builder = Arc::new(Mutex::new(UserBusinessDiBuilder::new(logger.clone())));

    user_business_builder
        .lock()
        .unwrap()
        .authenticated_user_repository(authenticated_user_repository)
        .anonymous_user_repository(anonymous_user_repository)
        .anonymous_binding_repository(anonymous_binding_repository)
        .session_repository(session_repository)
        .token_router(token_router)
        .member_authenticator(member_authenticator);

    let pbkdf2 = Arc::new(Pbkdf2::new());

    let ua_di = UserAccountBusinessDiBuilder::new(
        user_business_builder.clone(),
        "Argentum ToolKit. Example of CLI application",
        3600, // TTL 1h
        "http://localhost:8082/change-password/",
        id_factory.clone(),
        logger.clone(),
        notificator,
        pbkdf2,
    )
    .mock()
    .with_default_services(
        password_credential_repository,
        restore_password_token_repository,
        guest_token_generator,
        member_token_generator,
    )
    .build()?;

    let u_di = user_business_builder.lock().unwrap().build()?;

    Ok(DiC::new(
        id_factory,
        ua_di.anonymous_registers_uc,
        ua_di.user_logins_with_password_uc,
        ua_di.user_registers_with_password_uc,
        u_di.user_authenticates_with_token_uc,
        logger,
    ))
}
