use crate::repository::password_credential_checker::{
    PasswordCredentialChecker, PasswordCredentialCheckerError,
};
use crate::token::GeneratorTrait;
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_user_business::entity::anonymous_binding::AnonymousBinding;
use argentum_user_business::entity::session::Session;
use argentum_user_business::entity::user::{AnonymousUser, UserTrait};
use argentum_user_business::repository::anonymous_binding_repository::AnonymousBindingRepositoryTrait;
use argentum_user_business::repository::session_repository::{
    SessionRepositoryError, SessionRepositoryTrait,
};
use argentum_user_business::repository::user_repository::{
    AuthenticatedUserRepositoryTrait, ExternalUserError,
};
use std::sync::Arc;

pub struct UserLoginsWithPasswordUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    user_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>,
    anonymous_binding_repository: Arc<dyn AnonymousBindingRepositoryTrait<Id = I>>,
    session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
    credential_checker: Arc<PasswordCredentialChecker<I>>,
    id_factory: Arc<dyn IdFactory<Id = I>>,
    token_generator: Arc<dyn GeneratorTrait<Id = I>>,
    logger: Arc<L>,
}

impl<I, L> UserLoginsWithPasswordUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        user_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>,
        anonymous_binding_repository: Arc<dyn AnonymousBindingRepositoryTrait<Id = I>>,
        session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
        credential_checker: Arc<PasswordCredentialChecker<I>>,
        id_factory: Arc<dyn IdFactory<Id = I>>,
        token_generator: Arc<dyn GeneratorTrait<Id = I>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            user_repository,
            anonymous_binding_repository,
            session_repository,
            credential_checker,
            id_factory,
            token_generator,
            logger,
        }
    }

    pub async fn execute(
        &self,
        anonymous: Option<AnonymousUser<I>>,
        email: EmailAddress,
        password: &str,
    ) -> Result<Session<I>, LoginError> {
        let result = self.user_repository.find_by_email(&email).await?;

        let user = match result {
            Some(u) => u,
            None => return Err(LoginError::WrongEmailOrPassword),
        };

        if !self.credential_checker.check(user.id(), password).await? {
            return Err(LoginError::WrongEmailOrPassword);
        }

        let session = Session::new(
            self.id_factory.create(),
            user.id(),
            &self.token_generator.generate(user.id),
        );

        self.session_repository.save(&session).await?;

        if let Some(anonymous) = anonymous {
            match self
                .session_repository
                .delete_users_sessions(anonymous.id())
                .await
            {
                Ok(_) => self.logger.info("Anonymous session deleted"),
                Err(_) => self.logger.warning("Anonymous session is not deleted"),
            };

            let binding = AnonymousBinding::new(user.id(), anonymous.id());
            match self.anonymous_binding_repository.save(&binding).await {
                Ok(_) => self.logger.info("Anonymous binding saved"),
                Err(_) => self.logger.warning("Anonymous binding is not saved"),
            }
        }

        Result::Ok(session)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Can't save session")]
    SaveSession(
        #[source]
        #[from]
        SessionRepositoryError,
    ),

    #[error("Can't get an user. DB error")]
    GetUserError(
        #[source]
        #[from]
        ExternalUserError,
    ),

    #[error("Can't get an user. DB error")]
    PasswordCredentialCheckerError(
        #[source]
        #[from]
        PasswordCredentialCheckerError,
    ),

    #[error("Wrong email or password")]
    WrongEmailOrPassword,
}

#[cfg(test)]
mod test {
    use crate::entity::credential::PasswordCredential;
    use crate::mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock;
    use crate::mock::token::TokenGeneratorMock;
    use crate::repository::password_credential_checker::PasswordCredentialChecker;
    use crate::repository::password_credential_writer::PasswordCredentialWriter;
    use crate::repository::password_credential_writer::PasswordCredentialWriterTrait;
    use crate::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
    use argentum_encryption_business::mock::password::{EncryptorMock, ValidatorMock};
    use argentum_encryption_business::password::Encryptor;
    use argentum_log_business::{DefaultLogger, Level, StdoutWriter};
    use argentum_standard_business::data_type::email::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::data_type::builder::NameBuilder;
    use argentum_user_business::entity::user::{AnonymousUser, AuthenticatedUser};
    use argentum_user_business::mock::repository::anonymous_binding_repository_mock::AnonymousBindingRepositoryMock;
    use argentum_user_business::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
    use argentum_user_business::mock::repository::session_repository_mock::SessionRepositoryMock;
    use argentum_user_business::repository::anonymous_binding_repository::AnonymousBindingRepositoryTrait;
    use argentum_user_business::repository::user_repository::AuthenticatedUserRepositoryTrait;

    use std::error::Error;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_user_logins_with_password() -> Result<(), Box<dyn Error>> {
        let user_repository = Arc::new(AuthenticatedUserRepositoryMock::new());
        let anonymous_binding_repository = Arc::new(AnonymousBindingRepositoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let credential_repository = Arc::new(PasswordCredentialRepositoryMock::new());
        let validator = Arc::new(ValidatorMock::new());
        let credential_checker = Arc::new(PasswordCredentialChecker::new(
            credential_repository.clone(),
            validator.clone(),
        ));
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_generator = Arc::new(TokenGeneratorMock::new());
        let credential_writer = PasswordCredentialWriter::new(credential_repository.clone());

        let log_writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer.clone()));

        let uc = UserLoginsWithPasswordUc::new(
            user_repository.clone(),
            anonymous_binding_repository.clone(),
            session_repository.clone(),
            credential_checker.clone(),
            id_factory.clone(),
            token_generator.clone(),
            logger.clone(),
        );

        let id_factory = IdFactoryMock::new();

        let user_id = id_factory.create();
        let name = NameBuilder::new("Dionne")
            .last(Some("Morrison"))
            .try_build()
            .expect("Name should be valid");
        let email = EmailAddress::try_new("test@test-mail.com").expect("Email should be valid");
        let password = "12345";
        let user = AuthenticatedUser::new(user_id, name, email.clone());
        let encryptor = EncryptorMock::new();
        let (hashed_password, salt) = encryptor.encrypt(password)?;
        let cred = PasswordCredential::new(user_id, &hashed_password, &salt);

        user_repository
            .save(&user)
            .await
            .expect("Can't save a user");
        credential_writer.write(&cred).await?;

        let anonymous_id = id_factory.create();
        let anonymous = AnonymousUser::new(anonymous_id);

        let result = uc.execute(Some(anonymous), email, &password).await;

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
            Err(_) => {
                return Err("User can't login".into());
            }
        }
    }

    //TODO: negative test
}
