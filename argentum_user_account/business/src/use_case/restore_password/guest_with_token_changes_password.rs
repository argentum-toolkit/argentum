use crate::entity::credential::PasswordCredential;
use crate::repository::password_credential_writer::PasswordCredentialWriterTrait;
use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
use crate::use_case::restore_password::error::RestorePasswordError;

use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::repository::user_repository::MemberRepositoryTrait;
use secrecy::SecretString;
use std::sync::Arc;

pub struct GuestWithTokenChangesPasswordUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    user_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
    restore_password_token_repository: Arc<dyn RestorePasswordTokenRepositoryTrait<Id = I>>,
    credential_writer: Arc<dyn PasswordCredentialWriterTrait<Id = I>>,

    token_ttl: u32, //configurable ttl in seconds
    logger: Arc<L>,
}

impl<I, L> GuestWithTokenChangesPasswordUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        user_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
        restore_password_token_repository: Arc<dyn RestorePasswordTokenRepositoryTrait<Id = I>>,
        credential_writer: Arc<dyn PasswordCredentialWriterTrait<Id = I>>,
        token_ttl: u32,
        logger: Arc<L>,
    ) -> Self {
        Self {
            user_repository,
            restore_password_token_repository,
            credential_writer,
            token_ttl,
            logger,
        }
    }

    pub async fn execute(&self, token: &str, password: &str) -> Result<(), RestorePasswordError> {
        let restore_token = {
            let restore_token = self
                .restore_password_token_repository
                .find_by_token(token)
                .await;

            match restore_token {
                Err(e) => {
                    return Err(RestorePasswordError::TokenRepositoryError(e));
                }
                Ok(o) => match o {
                    Some(restore_token) => restore_token,
                    None => return Err(RestorePasswordError::TokenNotFoundError),
                },
            }
        };

        if restore_token.is_expired(self.token_ttl) {
            return Err(RestorePasswordError::TokenExpired);
        }

        let result = self.user_repository.find(restore_token.user_id).await;
        let user = match result {
            Err(err) => return Err(RestorePasswordError::GetUserError(err)),
            Ok(o) => match o {
                Some(user) => user,
                None => return Err(RestorePasswordError::UserNotFoundError),
            },
        };

        self.credential_writer.delete_for_user(user.id).await?;

        let pw = SecretString::new(password.into());
        let cred = PasswordCredential::new(user.id, pw);
        self.credential_writer.write(&cred).await?;

        if let Err(e) = self
            .restore_password_token_repository
            .delete_users_tokens(user.id)
            .await
        {
            self.logger
                .warning(format!("Restore password token is not deleted. {e:?}"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::restore_password_token::RestorePasswordToken;
    use crate::mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock;
    use crate::mock::repository::restore_password_token_repository_mock::RestorePasswordTokenRepositoryMock;
    use crate::mock::token::TokenGeneratorMock;
    use crate::repository::password_credential_repository::PasswordCredentialRepositoryTrait;
    use crate::repository::password_credential_writer::PasswordCredentialWriter;
    use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
    use crate::use_case::restore_password::error::RestorePasswordError;

    use crate::token::GeneratorTrait;
    use crate::use_case::restore_password::guest_with_token_changes_password::GuestWithTokenChangesPasswordUc;
    use argentum_encryption_business::mock::password::EncryptorMock;
    use argentum_log_business::{DefaultLogger, Level, StdoutWriter};
    use argentum_standard_business::data_type::EmailAddress;
    use argentum_standard_business::data_type::id::{IdFactory, UIntId};
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
    async fn guest_with_token_changes_password() -> Result<(), Box<dyn Error>> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_repository = Arc::new(RestorePasswordTokenRepositoryMock::new());
        let user_repository = Arc::new(MemberRepositoryMock::new());
        let token_generator: Arc<dyn GeneratorTrait<Id = UIntId>> =
            Arc::new(TokenGeneratorMock::new());
        let credential_repository = Arc::new(PasswordCredentialRepositoryMock::new(Arc::new(
            EncryptorMock::new(),
        )));
        let credential_writer =
            Arc::new(PasswordCredentialWriter::new(credential_repository.clone()));
        Arc::new(EncryptorMock::new());

        let writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Info, writer.clone()));
        let uc = GuestWithTokenChangesPasswordUc::new(
            user_repository.clone(),
            token_repository.clone(),
            credential_writer,
            100.clone(),
            logger.clone(),
        );

        let user_id = id_factory.create();
        let user_name = NameBuilder::new("Dionne".into())
            .last(Some("Morrison".into()))
            .try_build()
            .expect("Name should be valid");
        let email = EmailAddress::try_new("test@mail.com".into()).expect("Email should be valid");

        let user = Member::new(user_id, user_name, email);

        user_repository.create(&user).await?;

        let token = token_generator.generate(user.id);
        let token_id = id_factory.create();
        let restore_token = RestorePasswordToken::new(token_id, user.id.clone(), &token);

        token_repository.save(&restore_token).await?;

        let password = "234";
        let result = uc.execute(&token, password).await;

        if let Err(_) = result {
            return Err("Password is not changed".into());
        }

        if let Ok(None) = credential_repository.find_by_user_id(user.id).await {
            return Err("Can't find new password".into());
        }

        Ok(())
    }

    #[tokio::test]
    async fn guest_with_expired_token_changes_password() -> Result<(), Box<dyn Error>> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_repository = Arc::new(RestorePasswordTokenRepositoryMock::new());
        let user_repository = Arc::new(MemberRepositoryMock::new());
        let token_generator: Arc<dyn GeneratorTrait<Id = UIntId>> =
            Arc::new(TokenGeneratorMock::new());
        let credential_repository = Arc::new(PasswordCredentialRepositoryMock::new(Arc::new(
            EncryptorMock::new(),
        )));
        let credential_writer =
            Arc::new(PasswordCredentialWriter::new(credential_repository.clone()));

        let writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Info, writer.clone()));

        let uc = GuestWithTokenChangesPasswordUc::new(
            user_repository.clone(),
            token_repository.clone(),
            credential_writer.clone(),
            1,
            logger.clone(),
        );

        let user_id = id_factory.create();
        let user_name = NameBuilder::new("Dionne")
            .last(Some("Morrison"))
            .try_build()
            .expect("Name should be valid");

        let email = EmailAddress::try_new("test@mail.com").expect("Email should be valid");

        let user = Member::new(user_id, user_name.clone(), email.clone());

        user_repository.create(&user).await?;

        let token = token_generator.generate(user.id);
        let token_id = id_factory.create();
        let restore_token = RestorePasswordToken::new(token_id, user.id, &token);

        token_repository.save(&restore_token).await?;

        thread::sleep(Duration::from_secs(1));

        let password = "234";
        let result = uc.execute(&token, password).await;

        match result {
            Err(e) => match e {
                RestorePasswordError::TokenExpired => Ok(()),
                _ => Err("Should return 'Token Expired' error".into()),
            },
            _ => Err("Should return 'Token Expired' error.".into()),
        }
    }
}
