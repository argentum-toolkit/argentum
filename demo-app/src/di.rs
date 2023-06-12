use argentum_encryption_infrastructure::pbkdf2::Pbkdf2;
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_notification_business::mock::StdoutNotificator;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_account_business::di::UserAccountBusinessDiCBuilder;
use argentum_user_account_business::use_case::anonymous_registers::AnonymousRegistersUc;
use argentum_user_account_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use argentum_user_account_business::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
use argentum_user_account_business::use_case::user_registers_with_password::UserRegistersWithPasswordUc;
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
    let unique_id_factory = Arc::new(UniqueIdFactory::new());
    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let pbkdf2_password_encryptor = Arc::new(Pbkdf2::new());
    let notificator = Arc::new(StdoutNotificator::new());

    let ua_di = UserAccountBusinessDiCBuilder::new(
        unique_id_factory.clone(),
        pbkdf2_password_encryptor.clone(),
        pbkdf2_password_encryptor,
        logger.clone(),
        notificator,
    )
    .mock()
    .config(
        "Argentum ToolKit demo web application".to_string(),
        3600, // TTL 1h
        "http://localhost:8082/change-password/".to_string(),
    )
    .build();

    DiC::new(
        unique_id_factory,
        ua_di.anonymous_registers_uc,
        ua_di.user_logins_with_password_uc,
        ua_di.user_registers_with_password_uc,
        ua_di.user_authenticates_with_token_uc,
        logger,
    )
}
