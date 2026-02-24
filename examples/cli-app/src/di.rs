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
use argentum_user_account_business::use_case::guest_registers::GuestRegistersUc;
use argentum_user_account_business::use_case::member_logins_with_password::MemberLoginsWithPasswordUc;
use argentum_user_account_business::use_case::member_registers_with_password::MemberRegistersWithPasswordUc;
use argentum_user_business::di::UserBusinessDiBuilder;
use argentum_user_business::mock::repository::guest_binding_repository_mock::GuestBindingRepositoryMock;
use argentum_user_business::mock::repository::guest_repository_mock::GuestRepositoryMock;
use argentum_user_business::mock::repository::member_repository_mock::MemberRepositoryMock;
use argentum_user_business::mock::repository::session_repository_mock::SessionRepositoryMock;
use argentum_user_business::mock::service::{MemberAuthenticatorMock, TokenRouterMock};
use argentum_user_business::use_case::member_authenticates_with_token::MemberAuthenticatesWithTokenUc;
use std::sync::{Arc, Mutex};

pub struct DiC<L>
where
    L: LoggerTrait,
{
    // Public services
    pub id_factory: Arc<dyn IdFactory<Id = UIntId>>,
    pub guest_registers_uc: Arc<GuestRegistersUc<UIntId>>,
    pub member_logins_with_pw: Arc<MemberLoginsWithPasswordUc<UIntId, L>>,
    pub member_registers_with_pw: Arc<MemberRegistersWithPasswordUc<UIntId>>,
    pub user_authenticates_with_token: Arc<MemberAuthenticatesWithTokenUc<UIntId>>,
    pub logger: Arc<L>,
}

impl<L> DiC<L>
where
    L: LoggerTrait,
{
    pub fn new(
        id_factory: Arc<dyn IdFactory<Id = UIntId>>,
        guest_registers_uc: Arc<GuestRegistersUc<UIntId>>,
        member_logins_with_pw: Arc<MemberLoginsWithPasswordUc<UIntId, L>>,
        member_registers_with_pw: Arc<MemberRegistersWithPasswordUc<UIntId>>,
        user_authenticates_with_token: Arc<MemberAuthenticatesWithTokenUc<UIntId>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            id_factory,
            guest_registers_uc,
            member_logins_with_pw,
            member_registers_with_pw,
            user_authenticates_with_token,
            logger,
        }
    }
}

pub fn di_factory() -> Result<DiC<DefaultLogger<PrettyWriter>>, String> {
    let id_factory = Arc::new(IdFactoryMock::new());
    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let notifier = Arc::new(StdoutNotifier::new());

    let guest_binding_repository = Arc::new(GuestBindingRepositoryMock::<UIntId>::new());
    let guest_repository = Arc::new(GuestRepositoryMock::<UIntId>::new());
    let member_repository = Arc::new(MemberRepositoryMock::<UIntId>::new());
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
        .with_member_repository(member_repository)
        .with_guest_repository(guest_repository)
        .with_guest_binding_repository(guest_binding_repository)
        .with_session_repository(session_repository)
        .with_token_router(token_router)
        .with_member_authenticator(member_authenticator);

    let pbkdf2 = Arc::new(Pbkdf2::new());

    let ua_di = UserAccountBusinessDiBuilder::new(
        user_business_builder.clone(),
        "Argentum ToolKit. Example of CLI application",
        3600, // TTL 1h
        "http://localhost:8082/change-password/",
        id_factory.clone(),
        logger.clone(),
        notifier,
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
        ua_di.guest_registers_uc,
        ua_di.member_logins_with_password_uc,
        ua_di.member_registers_with_password_uc,
        u_di.user_authenticates_with_token_uc,
        logger,
    ))
}
