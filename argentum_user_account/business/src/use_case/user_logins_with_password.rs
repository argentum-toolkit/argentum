use crate::service::{PasswordToTokenExchangerError, PasswordToTokenExchangerTrait};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::entity::session::Session;
use argentum_user_business::entity::user::AnonymousUser;
use argentum_user_business::service::GuestBinder;
use secrecy::SecretString;
use std::error::Error;
use std::sync::Arc;

pub struct UserLoginsWithPasswordUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    password_to_token_exchanger: Arc<dyn PasswordToTokenExchangerTrait<Id = I>>,
    guest_binder: Arc<GuestBinder<I, L>>,
}

impl<I, L> UserLoginsWithPasswordUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        password_to_token_exchanger: Arc<dyn PasswordToTokenExchangerTrait<Id = I>>,
        guest_binder: Arc<GuestBinder<I, L>>,
    ) -> Self {
        Self {
            password_to_token_exchanger,
            guest_binder,
        }
    }

    pub async fn execute(
        &self,
        guest: Option<AnonymousUser<I>>,
        email: EmailAddress,
        password: SecretString,
    ) -> Result<Session<I>, LoginError> {
        let result = self
            .password_to_token_exchanger
            .exchange_password(&email, &password)
            .await;

        let session = match result {
            Ok(s) => s,
            Err(e) => match e {
                PasswordToTokenExchangerError::WrongEmailOrPassword => {
                    return Err(LoginError::WrongEmailOrPassword);
                }
                _ => {
                    return Err(LoginError::Infrastructure(Some(Arc::new(e))));
                }
            },
        };

        self.guest_binder
            .bind_and_logout_guest(guest, session.user_id)
            .await;

        Result::Ok(session)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Infrastructure error")]
    Infrastructure(
        #[source]
        #[from]
        Option<Arc<dyn Error + Send + Sync>>,
    ),

    #[error("Wrong email or password")]
    WrongEmailOrPassword,
}

#[cfg(test)]
mod test {
    use crate::mock::service::PasswordToTokenExchangerMock;
    use crate::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
    use argentum_log_business::{DefaultLogger, Level, StdoutWriter};
    use argentum_standard_business::data_type::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::entity::session::Session;
    use argentum_user_business::entity::user::AnonymousUser;
    use argentum_user_business::mock::repository::anonymous_binding_repository_mock::AnonymousBindingRepositoryMock;
    use argentum_user_business::mock::repository::session_repository_mock::SessionRepositoryMock;
    use argentum_user_business::repository::anonymous_binding_repository::AnonymousBindingRepositoryTrait;
    use argentum_user_business::service::GuestBinder;

    use std::error::Error;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_user_logins_with_password() -> Result<(), Box<dyn Error>> {
        let id_factory = Arc::new(IdFactoryMock::new());

        let anonymous_binding_repository = Arc::new(AnonymousBindingRepositoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());

        let log_writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer.clone()));

        let exchanger = Arc::new(PasswordToTokenExchangerMock::new());

        let guest_binder = Arc::new(GuestBinder::new(
            anonymous_binding_repository.clone(),
            session_repository.clone(),
            logger.clone(),
        ));

        let uc = UserLoginsWithPasswordUc::new(exchanger.clone(), guest_binder);

        let user_id = id_factory.create();
        let email = EmailAddress::try_new("test@test-mail.com").expect("Email should be valid");

        let password = "12345";

        exchanger.seed(
            email.clone(),
            password.into(),
            Ok(Session::new(1, user_id, "test_token")),
        )?;

        let anonymous_id = id_factory.create();
        let anonymous = AnonymousUser::new(anonymous_id);

        let result = uc.execute(Some(anonymous), email, password.into()).await;

        match result {
            Ok(s) => {
                assert_eq!(s.user_id, user_id);

                let binding = anonymous_binding_repository
                    .find_by_user_id(user_id)
                    .await?
                    .expect("Should be Some");
                assert_eq!(binding.anonymous_id, anonymous_id);

                return Ok(());
            }
            Err(e) => {
                return Err(format!("User can't login. Error: `{}`", e).into());
            }
        }
    }

    //TODO: negative test
}
