use crate::{
    event::BeforeMemberDeletedAccount,
    repository::{
        credential_writer::CredentialWriterError,
        email_confirmation_token_repository::EmailConfirmationTokenRepositoryTrait,
        password_credential_checker::{PasswordCredentialChecker, PasswordCredentialCheckerError},
        password_credential_writer::PasswordCredentialWriterTrait,
        restore_password_token_repository::RestorePasswordTokenRepositoryTrait,
    },
};
use argentum_event_business::{EventDispatcher, EventDispatcherError};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::repository::{
    session_repository::{SessionRepositoryError, SessionRepositoryTrait},
    user_repository::{ExternalUserError, MemberRepositoryTrait},
};
use secrecy::SecretString;
use std::sync::Arc;

pub struct MemberDeletesAccountUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    password_checker: Arc<PasswordCredentialChecker<I>>,
    session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
    email_confirmation_token_repository: Arc<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>,
    restore_password_token_repository: Arc<dyn RestorePasswordTokenRepositoryTrait<Id = I>>,
    credential_writer: Arc<dyn PasswordCredentialWriterTrait<Id = I>>,
    member_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
    event_dispatcher: Arc<EventDispatcher>,
    logger: Arc<L>,
}

impl<I, L> MemberDeletesAccountUc<I, L>
where
    I: Id + 'static,
    L: LoggerTrait,
{
    pub fn new(
        password_checker: Arc<PasswordCredentialChecker<I>>,
        session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
        email_confirmation_token_repository: Arc<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>,
        restore_password_token_repository: Arc<dyn RestorePasswordTokenRepositoryTrait<Id = I>>,
        credential_writer: Arc<dyn PasswordCredentialWriterTrait<Id = I>>,
        member_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
        event_dispatcher: Arc<EventDispatcher>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            password_checker,
            session_repository,
            email_confirmation_token_repository,
            restore_password_token_repository,
            credential_writer,
            member_repository,
            event_dispatcher,
            logger,
        }
    }

    pub async fn execute(
        &self,
        member_id: I,
        password: SecretString,
    ) -> Result<(), MemberDeletesAccountError> {
        if !self.password_checker.check(member_id, &password).await? {
            return Err(MemberDeletesAccountError::WrongPassword);
        }

        self.event_dispatcher
            .dispatch(BeforeMemberDeletedAccount::new(member_id))
            .await?;

        self.session_repository
            .delete_users_sessions(member_id)
            .await?;

        if let Err(e) = self
            .email_confirmation_token_repository
            .delete_users_tokens(member_id)
            .await
        {
            self.logger
                .warning(format!("Email confirmation token is not deleted. {e:?}"));
        }

        if let Err(e) = self
            .restore_password_token_repository
            .delete_users_tokens(member_id)
            .await
        {
            self.logger
                .warning(format!("Restore password token is not deleted. {e:?}"));
        }

        self.credential_writer.delete_for_user(member_id).await?;

        self.member_repository.delete(member_id).await?;

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MemberDeletesAccountError {
    #[error("Wrong password")]
    WrongPassword,

    #[error("Can't check password")]
    PasswordCheck(
        #[source]
        #[from]
        PasswordCredentialCheckerError,
    ),

    #[error("Can't delete sessions")]
    Session(
        #[source]
        #[from]
        SessionRepositoryError,
    ),

    #[error("Can't delete credentials")]
    Credential(
        #[source]
        #[from]
        CredentialWriterError,
    ),

    #[error("Can't delete member")]
    Member(
        #[source]
        #[from]
        ExternalUserError,
    ),

    #[error("Event dispatcher error")]
    EventDispatcher(
        #[source]
        #[from]
        EventDispatcherError,
    ),
}

#[cfg(test)]
mod test {
    use crate::entity::credential::PasswordCredential;
    use crate::entity::{EmailConfirmationToken, RestorePasswordToken};
    use crate::event::BeforeMemberDeletedAccount;
    use crate::mock::repository::email_confirmation_token_repository_mock::EmailConfirmationTokenRepositoryMock;
    use crate::mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock;
    use crate::mock::repository::restore_password_token_repository_mock::RestorePasswordTokenRepositoryMock;
    use crate::repository::email_confirmation_token_repository::EmailConfirmationTokenRepositoryTrait;
    use crate::repository::password_credential_checker::PasswordCredentialChecker;
    use crate::repository::password_credential_repository::PasswordCredentialRepositoryTrait;
    use crate::repository::password_credential_writer::{
        PasswordCredentialWriter, PasswordCredentialWriterTrait,
    };
    use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
    use crate::use_case::member_deletes_account::{
        MemberDeletesAccountError, MemberDeletesAccountUc,
    };
    use argentum_encryption_business::mock::password::{EncryptorMock, ValidatorMock};
    use argentum_event_business::{Event, EventDispatcher, EventListener};
    use argentum_log_business::{DefaultLogger, Level, StdoutWriter};
    use argentum_standard_business::data_type::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::data_type::builder::NameBuilder;
    use argentum_user_business::entity::session::Session;
    use argentum_user_business::entity::user::Member;
    use argentum_user_business::mock::repository::broken::member_repository_mock::MemberRepositoryMockWihBrokenSave;
    use argentum_user_business::mock::repository::member_repository_mock::MemberRepositoryMock;
    use argentum_user_business::mock::repository::session_repository_mock::SessionRepositoryMock;
    use argentum_user_business::repository::session_repository::SessionRepositoryTrait;
    use argentum_user_business::repository::user_repository::MemberRepositoryTrait;
    use async_trait::async_trait;
    use secrecy::SecretString;
    use std::error::Error;
    use std::sync::{Arc, Mutex};

    #[derive(Default)]
    struct EventNameListener {
        names: Mutex<Vec<&'static str>>,
    }

    #[async_trait]
    impl EventListener for EventNameListener {
        async fn listen(&self, event: &dyn Event) {
            self.names.lock().unwrap().push(event.name());
        }
    }

    #[tokio::test]
    async fn member_deletes_account() -> Result<(), Box<dyn Error>> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let email_token_repository = Arc::new(EmailConfirmationTokenRepositoryMock::new());
        let restore_token_repository = Arc::new(RestorePasswordTokenRepositoryMock::new());
        let member_repository = Arc::new(MemberRepositoryMock::new());
        let credential_repository = Arc::new(PasswordCredentialRepositoryMock::new(Arc::new(
            EncryptorMock::new(),
        )));
        let credential_writer =
            Arc::new(PasswordCredentialWriter::new(credential_repository.clone()));
        let password_checker = Arc::new(PasswordCredentialChecker::new(
            credential_repository.clone(),
            Arc::new(ValidatorMock::new()),
        ));

        let event_dispatcher = Arc::new(EventDispatcher::new());
        let listener = Arc::new(EventNameListener::default());
        event_dispatcher.register_listener(listener.clone())?;

        let writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Info, writer.clone()));

        let uc = MemberDeletesAccountUc::new(
            password_checker,
            session_repository.clone(),
            email_token_repository.clone(),
            restore_token_repository.clone(),
            credential_writer.clone(),
            member_repository.clone(),
            event_dispatcher,
            logger,
        );

        let user_id = id_factory.create();
        let user_name = NameBuilder::new("Dionne")
            .last(Some("Morrison"))
            .try_build()
            .expect("Name should be valid");
        let email = EmailAddress::try_new("test@mail.com").expect("Email should be valid");
        let member = Member::new(user_id, user_name, email, true);
        member_repository.create(&member).await?;

        let password = "12345";
        credential_writer
            .write(&PasswordCredential::new(user_id, password.into()))
            .await?;

        session_repository
            .save(&Session::new(id_factory.create(), user_id, "session_token"))
            .await?;
        email_token_repository
            .save(&EmailConfirmationToken::new(
                id_factory.create(),
                user_id,
                "email_token",
            ))
            .await?;
        restore_token_repository
            .save(&RestorePasswordToken::new(
                id_factory.create(),
                user_id,
                "restore_token",
            ))
            .await?;

        uc.execute(user_id, password.into()).await?;

        assert!(
            session_repository
                .find_by_token("session_token")
                .await?
                .is_none()
        );
        assert!(
            email_token_repository
                .find_by_token("email_token")
                .await?
                .is_none()
        );
        assert!(
            restore_token_repository
                .find_by_token("restore_token")
                .await?
                .is_none()
        );
        assert!(
            credential_repository
                .find_by_user_id(user_id)
                .await?
                .is_none()
        );
        assert!(member_repository.find(user_id).await?.is_none());

        let names = listener.names.lock().unwrap();
        assert_eq!(
            names.as_slice(),
            &[BeforeMemberDeletedAccount::new(user_id).name()]
        );

        Ok(())
    }

    #[tokio::test]
    async fn member_deletes_account_with_wrong_password() -> Result<(), Box<dyn Error>> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let email_token_repository = Arc::new(EmailConfirmationTokenRepositoryMock::new());
        let restore_token_repository = Arc::new(RestorePasswordTokenRepositoryMock::new());
        let member_repository = Arc::new(MemberRepositoryMock::new());
        let credential_repository = Arc::new(PasswordCredentialRepositoryMock::new(Arc::new(
            EncryptorMock::new(),
        )));
        let credential_writer =
            Arc::new(PasswordCredentialWriter::new(credential_repository.clone()));
        let password_checker = Arc::new(PasswordCredentialChecker::new(
            credential_repository.clone(),
            Arc::new(ValidatorMock::new()),
        ));

        let event_dispatcher = Arc::new(EventDispatcher::new());
        let listener = Arc::new(EventNameListener::default());
        event_dispatcher.register_listener(listener.clone())?;

        let writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Info, writer.clone()));

        let uc = MemberDeletesAccountUc::new(
            password_checker,
            session_repository.clone(),
            email_token_repository.clone(),
            restore_token_repository.clone(),
            credential_writer.clone(),
            member_repository.clone(),
            event_dispatcher,
            logger,
        );

        let user_id = id_factory.create();
        let user_name = NameBuilder::new("Dionne")
            .last(Some("Morrison"))
            .try_build()
            .expect("Name should be valid");
        let email = EmailAddress::try_new("test@mail.com").expect("Email should be valid");
        let member = Member::new(user_id, user_name, email, true);
        member_repository.create(&member).await?;

        credential_writer
            .write(&PasswordCredential::new(user_id, "12345".into()))
            .await?;
        session_repository
            .save(&Session::new(id_factory.create(), user_id, "session_token"))
            .await?;

        let result = uc.execute(user_id, SecretString::from("wrong")).await;

        match result {
            Err(MemberDeletesAccountError::WrongPassword) => {}
            _ => return Err("Should return 'WrongPassword' error".into()),
        }

        assert!(
            session_repository
                .find_by_token("session_token")
                .await?
                .is_some()
        );
        assert!(
            credential_repository
                .find_by_user_id(user_id)
                .await?
                .is_some()
        );
        assert!(member_repository.find(user_id).await?.is_some());
        assert!(listener.names.lock().unwrap().is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn member_deletes_account_with_broken_member_repository() -> Result<(), Box<dyn Error>> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let email_token_repository = Arc::new(EmailConfirmationTokenRepositoryMock::new());
        let restore_token_repository = Arc::new(RestorePasswordTokenRepositoryMock::new());
        let member_repository = Arc::new(MemberRepositoryMockWihBrokenSave::new());
        let credential_repository = Arc::new(PasswordCredentialRepositoryMock::new(Arc::new(
            EncryptorMock::new(),
        )));
        let credential_writer =
            Arc::new(PasswordCredentialWriter::new(credential_repository.clone()));
        let password_checker = Arc::new(PasswordCredentialChecker::new(
            credential_repository.clone(),
            Arc::new(ValidatorMock::new()),
        ));

        let event_dispatcher = Arc::new(EventDispatcher::new());
        let listener = Arc::new(EventNameListener::default());
        event_dispatcher.register_listener(listener.clone())?;

        let writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Info, writer.clone()));

        let uc = MemberDeletesAccountUc::new(
            password_checker,
            session_repository,
            email_token_repository,
            restore_token_repository,
            credential_writer.clone(),
            member_repository,
            event_dispatcher,
            logger,
        );

        let user_id = id_factory.create();
        let password = "12345";
        credential_writer
            .write(&PasswordCredential::new(user_id, password.into()))
            .await?;

        let result = uc.execute(user_id, password.into()).await;

        match result {
            Err(MemberDeletesAccountError::Member(_)) => {}
            _ => return Err("Should return 'Member' infrastructure error".into()),
        }

        let names = listener.names.lock().unwrap();
        assert_eq!(
            names.as_slice(),
            &[BeforeMemberDeletedAccount::new(user_id).name()]
        );

        Ok(())
    }
}
