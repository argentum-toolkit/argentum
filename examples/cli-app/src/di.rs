use argentum_encryption_business::mock::password::EncryptorMock;
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_notification_business::mock::StdoutNotificator;
use argentum_standard_business::data_type::id::{IdFactory, UIntId};
use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
use argentum_user_account_business::di::UserAccountBusinessDiCBuilder;
use argentum_user_account_business::mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock;
use argentum_user_account_business::mock::repository::restore_password_token_repository_mock::RestorePasswordTokenRepositoryMock;
use argentum_user_account_business::mock::service::PasswordToTokenExchangerMock;
use argentum_user_account_business::mock::token::TokenGeneratorMock;
use argentum_user_account_business::use_case::anonymous_registers::AnonymousRegistersUc;
use argentum_user_account_business::use_case::member_confirms_email::DefaultMemberConfirmsEmailUc;
use argentum_user_account_business::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
use argentum_user_account_business::use_case::user_registers_with_password::UserRegistersWithPasswordUc;
use argentum_user_business::mock::repository::anonymous_binding_repository_mock::AnonymousBindingRepositoryMock;
use argentum_user_business::mock::repository::anonymous_user_repository_mock::AnonymousUserRepositoryMock;
use argentum_user_business::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
use argentum_user_business::mock::repository::session_repository_mock::SessionRepositoryMock;
use argentum_user_business::mock::service::{MemberAuthenticatorMock, TokenRouterMock};
use argentum_user_business::service::GuestBinder;
use argentum_user_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use std::sync::Arc;

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

    let notificator = Arc::new(StdoutNotificator::new());

    let anonymous_binding_repository = Arc::new(AnonymousBindingRepositoryMock::new());
    let anonymous_user_repository = Arc::new(AnonymousUserRepositoryMock::new());
    let authenticated_user_repository = Arc::new(AuthenticatedUserRepositoryMock::new());
    let session_repository = Arc::new(SessionRepositoryMock::new());

    let encryptor = Arc::new(EncryptorMock::new());
    let password_credential_repository = Arc::new(PasswordCredentialRepositoryMock::new(encryptor));
    let password_to_token_exchanger = Arc::new(PasswordToTokenExchangerMock::new());
    let restore_password_token_repository = Arc::new(RestorePasswordTokenRepositoryMock::new());
    let guest_token_generator = Arc::new(TokenGeneratorMock::new());
    let member_token_generator = Arc::new(TokenGeneratorMock::new());
    let token_router = Arc::new(TokenRouterMock::new());
    let member_authenticator = Arc::new(MemberAuthenticatorMock::new());
    let member_confirms_email_uc = Arc::new(DefaultMemberConfirmsEmailUc::new(
        authenticated_user_repository.clone(),
        logger.clone(),
    ));

    let gust_binder = Arc::new(GuestBinder::new(
        anonymous_binding_repository,
        session_repository.clone(),
        logger.clone(),
    ));

    let ua_di = UserAccountBusinessDiCBuilder::new(id_factory.clone(), logger.clone(), notificator)
        .mock()
        .config(
            "Argentum ToolKit. Example of CLI application",
            3600, // TTL 1h
            "http://localhost:8082/change-password/",
        )
        .services(
            gust_binder,
            anonymous_user_repository,
            authenticated_user_repository,
            session_repository,
            password_credential_repository,
            password_to_token_exchanger,
            restore_password_token_repository,
            guest_token_generator,
            member_token_generator,
            token_router,
            member_authenticator,
            member_confirms_email_uc,
        )
        .build()?;

    Ok(DiC::new(
        id_factory,
        ua_di.anonymous_registers_uc,
        ua_di.user_logins_with_password_uc,
        ua_di.user_registers_with_password_uc,
        ua_di.user_authenticates_with_token_uc,
        logger,
    ))
}
