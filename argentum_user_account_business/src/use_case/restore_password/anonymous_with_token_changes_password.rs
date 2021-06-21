use crate::entity::credential::PasswordCredential;
use crate::repository::password_credential_writer::PasswordCredentialWriterTrait;
use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
use crate::use_case::restore_password::error::RestorePasswordError;
use argentum_encryption_business::password::Encryptor;
use argentum_user_business::repository::user_repository::AuthenticatedUserRepositoryTrait;

pub struct AnonymousWithTokenChangesPassword<'s> {
    user_repository: &'s dyn AuthenticatedUserRepositoryTrait,
    restore_password_token_repository: &'s dyn RestorePasswordTokenRepositoryTrait,
    credential_writer: &'s dyn PasswordCredentialWriterTrait<'s>,
    encryptor: &'s dyn Encryptor,
    token_ttl: u32, //configurable ttl in seconds
}

impl<'s> AnonymousWithTokenChangesPassword<'s> {
    pub fn new(
        user_repository: &'s dyn AuthenticatedUserRepositoryTrait,
        restore_password_token_repository: &'s dyn RestorePasswordTokenRepositoryTrait,
        encryptor: &'s dyn Encryptor,
        credential_writer: &'s dyn PasswordCredentialWriterTrait<'s>,
        token_ttl: u32,
    ) -> AnonymousWithTokenChangesPassword<'s> {
        AnonymousWithTokenChangesPassword {
            user_repository,
            restore_password_token_repository,
            credential_writer,
            encryptor,
            token_ttl,
        }
    }

    pub fn execute(&self, token: String, password: String) -> Result<(), RestorePasswordError> {
        let restore_token = {
            let restore_token = self.restore_password_token_repository.find_by_token(token);

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

        let result = self.user_repository.find(&restore_token.user_id);
        let user = match result {
            Err(err) => return Err(RestorePasswordError::GetUserError(err)),
            Ok(o) => match o {
                Some(user) => user,
                None => return Err(RestorePasswordError::UserNotFoundError),
            },
        };

        self.credential_writer.delete_for_user(&user.id);

        //save credentials
        let (hashed_password, salt) = self.encryptor.encrypt(&password)?;

        let cred = PasswordCredential::new(user.id.clone(), hashed_password, salt);
        self.credential_writer.write(Box::new(cred));

        if self
            .restore_password_token_repository
            .delete_users_tokens(&user.id)
            .is_err()
        {
            //TODO: log error
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
    use crate::repository::password_credential_repository::PasswordCredentialRepository;
    use crate::repository::password_credential_writer::PasswordCredentialWriter;
    use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
    use crate::use_case::restore_password::anonymous_with_token_changes_password::AnonymousWithTokenChangesPassword;
    use crate::use_case::restore_password::error::RestorePasswordError;
    use argentum_encryption_business::mock::password::EncryptorMock;
    use argentum_standard_business::data_type::email::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::entity::user::AuthenticatedUser;
    use argentum_user_business::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
    use argentum_user_business::repository::user_repository::AuthenticatedUserRepositoryTrait;
    use argentum_user_business::token::GeneratorTrait;
    use argentum_user_business::value_object::name::Name;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn anonymous_changes_password_with_token() -> Result<(), &'static str> {
        let id_factory = IdFactoryMock::new();
        let token_repository = RestorePasswordTokenRepositoryMock::new();
        let user_repository = AuthenticatedUserRepositoryMock::new();
        let token_generator: &dyn GeneratorTrait = &TokenGeneratorMock::new();
        let credential_repository = PasswordCredentialRepositoryMock::new();
        let credential_writer = PasswordCredentialWriter::new(&credential_repository);
        let encryptor = EncryptorMock::new();

        let uc = AnonymousWithTokenChangesPassword::new(
            &user_repository,
            &token_repository,
            &encryptor,
            &credential_writer,
            100,
        );

        let user_id = id_factory.create();
        let user_name = Name::new("Dionne".to_string(), "Morrison".to_string()).unwrap();
        let email = EmailAddress::new("test@mail.com".to_string()).unwrap();

        let user = AuthenticatedUser::new(&user_id, user_name, email.clone());

        user_repository.save(&user).unwrap();

        let token = token_generator.generate(&user.id);
        let token_id = id_factory.create();
        let restore_token = RestorePasswordToken::new(token_id, user.id.clone(), token.clone());

        token_repository.save(&restore_token).unwrap();

        let password = "234".to_string();
        let result = uc.execute(token, password);

        if let Err(_) = result {
            return Err("Password is not changed");
        }

        if let None = credential_repository.find_by_user_id(&user_id) {
            return Err("Can't find new password");
        }

        Ok(())
    }

    #[test]
    fn anonymous_changes_password_with_expired_token() -> Result<(), &'static str> {
        let id_factory = IdFactoryMock::new();
        let token_repository = RestorePasswordTokenRepositoryMock::new();
        let user_repository = AuthenticatedUserRepositoryMock::new();
        let token_generator: &dyn GeneratorTrait = &TokenGeneratorMock::new();
        let credential_repository = PasswordCredentialRepositoryMock::new();
        let credential_writer = PasswordCredentialWriter::new(&credential_repository);
        let encryptor = EncryptorMock::new();

        let uc = AnonymousWithTokenChangesPassword::new(
            &user_repository,
            &token_repository,
            &encryptor,
            &credential_writer,
            1,
        );

        let user_id = id_factory.create();
        let user_name = Name::new("Dionne".to_string(), "Morrison".to_string()).unwrap();
        let email = EmailAddress::new("test@mail.com".to_string()).unwrap();

        let user = AuthenticatedUser::new(&user_id, user_name, email.clone());

        user_repository.save(&user).unwrap();

        let token = token_generator.generate(&user.id);
        let token_id = id_factory.create();
        let restore_token = RestorePasswordToken::new(token_id, user.id.clone(), token.clone());

        token_repository.save(&restore_token).unwrap();

        thread::sleep(Duration::from_secs(1));

        let password = "234".to_string();
        let result = uc.execute(token, password);

        match result {
            Err(e) => match e {
                RestorePasswordError::TokenExpired => Ok(()),
                _ => Err("Should return 'Token Expired' error"),
            },
            _ => Err("Should return 'Token Expired' error."),
        }
    }
}
