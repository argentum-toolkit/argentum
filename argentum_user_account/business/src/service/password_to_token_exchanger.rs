use std::{error::Error, sync::Arc};

use argentum_standard_business::data_type::{
    EmailAddress,
    id::{Id, IdFactory},
};
use argentum_user_business::{
    entity::session::Session,
    repository::{
        session_repository::{SessionRepositoryError, SessionRepositoryTrait},
        user_repository::{ExternalUserError, MemberRepositoryTrait},
    },
};

use async_trait::async_trait;

use secrecy::SecretString;

use crate::{
    repository::password_credential_checker::{
        PasswordCredentialChecker, PasswordCredentialCheckerError,
    },
    token::MemberGeneratorTrait,
};

pub type PasswordToTokenExchangeResult<I> = Result<Session<I>, PasswordToTokenExchangerError>;

#[async_trait]
pub trait PasswordToTokenExchangerTrait: Send + Sync {
    type Id: Id;

    async fn exchange_password(
        &self,
        email: &EmailAddress,
        password: &SecretString,
    ) -> PasswordToTokenExchangeResult<Self::Id>;
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum PasswordToTokenExchangerError {
    #[error("Wrong email or password")]
    WrongEmailOrPassword,

    #[error("Email is not confirmed. Please check your email inbox.")]
    EmailNotConfirmed,

    #[error("Can't get an user. DB error")]
    GetUser(
        #[source]
        #[from]
        ExternalUserError,
    ),

    #[error("Can't check credentials. Internal error")]
    PasswordCredentialChecker(
        #[source]
        #[from]
        PasswordCredentialCheckerError,
    ),

    #[error("Can't check session. Internal error")]
    SessionRepository(
        #[source]
        #[from]
        SessionRepositoryError,
    ),

    #[error("Other infrastructure error")]
    Other(
        #[source]
        #[from]
        Option<Arc<dyn Error + Send + Sync>>,
    ),

    #[error("Other infrastructure error")]
    OtherString(String),
}

pub struct DefaultPasswordToTokenExchanger<I>
where
    I: Id,
{
    user_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
    session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
    credential_checker: Arc<PasswordCredentialChecker<I>>,
    id_factory: Arc<dyn IdFactory<Id = I>>,
    token_generator: Arc<dyn MemberGeneratorTrait<Id = I>>,
}

impl<I> DefaultPasswordToTokenExchanger<I>
where
    I: Id,
{
    pub fn new(
        user_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
        session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
        credential_checker: Arc<PasswordCredentialChecker<I>>,
        id_factory: Arc<dyn IdFactory<Id = I>>,
        token_generator: Arc<dyn MemberGeneratorTrait<Id = I>>,
    ) -> Self {
        Self {
            user_repository,
            session_repository,
            credential_checker,
            id_factory,
            token_generator,
        }
    }
}

#[async_trait]
impl<I> PasswordToTokenExchangerTrait for DefaultPasswordToTokenExchanger<I>
where
    I: Id,
{
    type Id = I;

    async fn exchange_password(
        &self,
        email: &EmailAddress,
        password: &SecretString,
    ) -> PasswordToTokenExchangeResult<Self::Id> {
        let result = self.user_repository.find_by_email(email).await?;

        let user = match result {
            Some(u) => u,
            None => return Err(PasswordToTokenExchangerError::WrongEmailOrPassword),
        };

        if !user.email_confirmed {
            return Err(PasswordToTokenExchangerError::EmailNotConfirmed);
        }

        if !self.credential_checker.check(user.id, password).await? {
            return Err(PasswordToTokenExchangerError::WrongEmailOrPassword);
        }

        let session = Session::new(
            self.id_factory.create(),
            user.id,
            &self.token_generator.generate(user.id),
        );

        self.session_repository.save(&session).await?;

        Ok(session)
    }
}
