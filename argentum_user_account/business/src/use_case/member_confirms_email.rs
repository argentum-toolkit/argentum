use crate::repository::email_confirmation_token_repository::EmailConfirmationTokenRepositoryTrait;
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::repository::user_repository::MemberRepositoryTrait;
use async_trait::async_trait;
use std::{error::Error, sync::Arc};

#[async_trait]
pub trait MemberConfirmsEmailUcTrait: Send + Sync {
    type Id: Id;

    async fn execute(&self, token: &str) -> Result<(), MemberConfirmsEmailError>;
}

pub struct DefaultMemberConfirmsEmailUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    member_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
    email_confirmation_token_repository: Arc<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>,
    logger: Arc<L>,
    token_ttl: u32,
}

impl<I, L> DefaultMemberConfirmsEmailUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        member_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
        email_confirmation_token_repository: Arc<dyn EmailConfirmationTokenRepositoryTrait<Id = I>>,
        logger: Arc<L>,
        token_ttl: u32,
    ) -> Self {
        Self {
            member_repository,
            email_confirmation_token_repository,
            logger,
            token_ttl,
        }
    }
}

#[async_trait]
impl<I, L> MemberConfirmsEmailUcTrait for DefaultMemberConfirmsEmailUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    type Id = I;

    async fn execute(&self, token: &str) -> Result<(), MemberConfirmsEmailError> {
        let confirmation_token = {
            let confirmation_token = self
                .email_confirmation_token_repository
                .find_by_token(token)
                .await;

            match confirmation_token {
                Err(e) => {
                    return Err(MemberConfirmsEmailError::Network(Arc::new(e)));
                }
                Ok(o) => match o {
                    Some(confirmation_token) => confirmation_token,
                    None => return Err(MemberConfirmsEmailError::TokenNotFoundError),
                },
            }
        };

        if confirmation_token.is_expired(self.token_ttl) {
            return Err(MemberConfirmsEmailError::TokenExpired);
        }

        let result = self
            .member_repository
            .find(confirmation_token.user_id)
            .await;

        let mut user = match result {
            Err(err) => return Err(MemberConfirmsEmailError::Network(Arc::new(err))),
            Ok(o) => match o {
                Some(user) => user,
                None => return Err(MemberConfirmsEmailError::UserNotFoundError),
            },
        };

        user.confirm_email();
        if let Err(e) = self.member_repository.update(&user).await {
            return Err(MemberConfirmsEmailError::Network(Arc::new(e)));
        }

        if let Err(e) = self
            .email_confirmation_token_repository
            .delete_users_tokens(user.id)
            .await
        {
            self.logger
                .warning(format!("Email confirmation token is not deleted. {e:?}"));
        }

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MemberConfirmsEmailError {
    #[error("User is not found")]
    UserNotFoundError,

    #[error("Token is not found")]
    TokenNotFoundError,

    #[error("Token expired")]
    TokenExpired,

    #[error("Can't confirm an email.")]
    Network(#[source] Arc<dyn Error + Send + Sync>),

    #[error("Can't confirm an email.")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use crate::entity::EmailConfirmationToken;

    use crate::mock::repository::email_confirmation_token_repository_mock::EmailConfirmationTokenRepositoryMock;
    use crate::repository::email_confirmation_token_repository::EmailConfirmationTokenRepositoryTrait;
    use crate::use_case::member_confirms_email::MemberConfirmsEmailError;
    use crate::use_case::member_confirms_email::MemberConfirmsEmailUcTrait;
    use argentum_log_business::{DefaultLogger, Level, StdoutWriter};
    use argentum_standard_business::data_type::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::data_type::builder::NameBuilder;
    use argentum_user_business::entity::user::Member;
    use argentum_user_business::mock::repository::member_repository_mock::MemberRepositoryMock;
    use argentum_user_business::repository::user_repository::MemberRepositoryTrait;
    use std::error::Error;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn member_confirms_email() -> Result<(), Box<dyn Error>> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_repository = Arc::new(EmailConfirmationTokenRepositoryMock::new());
        let member_repository = Arc::new(MemberRepositoryMock::new());

        let writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Info, writer.clone()));

        let uc = super::DefaultMemberConfirmsEmailUc::new(
            member_repository.clone(),
            token_repository.clone(),
            logger.clone(),
            100,
        );

        let user_id = id_factory.create();
        let user_name = NameBuilder::new("Dionne".into())
            .last(Some("Morrison".into()))
            .try_build()
            .expect("Name should be valid");
        let email = EmailAddress::try_new("test@mail.com".into()).expect("Email should be valid");

        let user = Member::new(user_id, user_name, email, false);

        member_repository.create(&user).await?;

        let token = "test_token";
        let token_id = id_factory.create();
        let confirmation_token = EmailConfirmationToken::new(token_id, user.id.clone(), token);

        token_repository.save(&confirmation_token).await?;

        let result = uc.execute(token).await;

        if let Err(_) = result {
            return Err("Email is not confirmed".into());
        }

        Ok(())
    }

    #[tokio::test]
    async fn member_confirms_email_with_expired_token() -> Result<(), Box<dyn Error>> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_repository = Arc::new(EmailConfirmationTokenRepositoryMock::new());
        let member_repository = Arc::new(MemberRepositoryMock::new());

        let writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Info, writer.clone()));

        let uc = super::DefaultMemberConfirmsEmailUc::new(
            member_repository.clone(),
            token_repository.clone(),
            logger.clone(),
            1,
        );

        let user_id = id_factory.create();
        let user_name = NameBuilder::new("Dionne".into())
            .last(Some("Morrison".into()))
            .try_build()
            .expect("Name should be valid");
        let email = EmailAddress::try_new("test@mail.com".into()).expect("Email should be valid");

        let user = Member::new(user_id, user_name, email, false);

        member_repository.create(&user).await?;

        let token = "test_token";
        let token_id = id_factory.create();
        let confirmation_token = EmailConfirmationToken::new(token_id, user.id, token);

        token_repository.save(&confirmation_token).await?;

        thread::sleep(Duration::from_secs(1));

        let result = uc.execute(token).await;

        match result {
            Err(e) => match e {
                MemberConfirmsEmailError::TokenExpired => Ok(()),
                _ => Err("Should return 'Token Expired' error".into()),
            },
            _ => Err("Should return 'Token Expired' error.".into()),
        }
    }
}
