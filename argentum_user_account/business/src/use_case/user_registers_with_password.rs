use crate::entity::credential::PasswordCredential;
use crate::repository::password_credential_writer::PasswordCredentialWriterTrait;

use argentum_encryption_business::password::{EncryptionError, Encryptor};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_user_business::entity::user::AuthenticatedUser;
use argentum_user_business::repository::user_repository::{
    AuthenticatedUserRepositoryTrait, ExternalUserError,
};
use argentum_user_business::value_object::name::Name;

use argentum_standard_business::data_type::id::Id;
use std::sync::Arc;

pub struct UserRegistersWithPasswordUc {
    user_repository: Arc<dyn AuthenticatedUserRepositoryTrait>,
    credential_writer: Arc<dyn PasswordCredentialWriterTrait>,
    encryptor: Arc<dyn Encryptor>,
}

impl UserRegistersWithPasswordUc {
    pub fn new(
        user_repository: Arc<dyn AuthenticatedUserRepositoryTrait>,
        credential_writer: Arc<dyn PasswordCredentialWriterTrait>,
        encryptor: Arc<dyn Encryptor>,
    ) -> UserRegistersWithPasswordUc {
        UserRegistersWithPasswordUc {
            user_repository,
            credential_writer,
            encryptor,
        }
    }

    pub fn execute(
        &self,
        id: Id,
        name: Name,
        email: EmailAddress,
        password: String,
    ) -> Result<AuthenticatedUser, RegistrationError> {
        let result = self.user_repository.find_by_email(&email);

        match result {
            Ok(o) => {
                if o.is_some() {
                    return Err(RegistrationError::EmailAlreadyExists);
                }
            }
            Err(e) => return Err(RegistrationError::SavingError(e)),
        };

        //save user
        let user = {
            //it is `temporary mutability` pattern
            let user = AuthenticatedUser::new(&id, name, email);
            let result = self.user_repository.save(&user);

            match result {
                Ok(_) => user,
                Err(e) => return Err(RegistrationError::SavingError(e)),
            }
        };

        //save credentials
        let (hashed_password, salt) = self.encryptor.encrypt(&password)?;

        let cred = PasswordCredential::new(id.clone(), hashed_password, salt);
        self.credential_writer.write(Box::new(cred));

        Ok(user)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RegistrationError {
    #[error("User with email Already exists")]
    EmailAlreadyExists,

    #[error("Can't encrypt password")]
    EncryptionError(#[from] EncryptionError),

    #[error("External user's error")]
    SavingError(#[from] ExternalUserError),
}

#[cfg(test)]
mod test {
    use crate::mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock;
    use crate::repository::password_credential_writer::PasswordCredentialWriter;
    use crate::use_case::user_registers_with_password::{
        RegistrationError, UserRegistersWithPasswordUc,
    };
    use argentum_encryption_business::mock::password::EncryptorMock;
    use argentum_standard_business::data_type::email::EmailAddress;
    use argentum_standard_business::data_type::id::{Id, IdFactory};
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
    use argentum_user_business::mock::repository::broken::authenticated_user_repository_mock::AuthenticatedUserRepositoryMockWihBrokenSave;
    use argentum_user_business::value_object::name::Name;
    use std::sync::Arc;

    #[test]
    fn test_user_registers_with_password() -> Result<(), &'static str> {
        let credential_repository = PasswordCredentialRepositoryMock::new();
        let credential_writer = PasswordCredentialWriter::new(Arc::new(credential_repository));
        let encryptor = EncryptorMock::new();
        let authenticated_user_repository = AuthenticatedUserRepositoryMock::new();
        let uc = UserRegistersWithPasswordUc::new(
            Arc::new(authenticated_user_repository),
            Arc::new(credential_writer),
            Arc::new(encryptor),
        );
        let id_factory = IdFactoryMock::new();

        let id: Id = id_factory.create();
        let name = Name::new(String::from("John"), String::from("Cooper")).unwrap();
        let email = EmailAddress::new(String::from("demo@test.com")).unwrap();
        let password = String::from("123");
        let result = uc.execute(id.clone(), name, email, password);

        match result {
            Ok(u) => {
                assert_eq!(u.id.to_string(), id.clone().to_string());

                return Ok(());
            }
            Err(_) => {
                return Err("Can't register an user");
            }
        }
    }

    #[test]
    fn test_user_registers_with_password_with_broken_user_repository() -> Result<(), &'static str> {
        let credential_repository = PasswordCredentialRepositoryMock::new();
        let credential_writer = PasswordCredentialWriter::new(Arc::new(credential_repository));
        let encryptor = EncryptorMock::new();
        let authenticated_user_repository = AuthenticatedUserRepositoryMockWihBrokenSave::new();
        let uc = UserRegistersWithPasswordUc::new(
            Arc::new(authenticated_user_repository),
            Arc::new(credential_writer),
            Arc::new(encryptor),
        );

        let id_factory = IdFactoryMock::new();

        let id: Id = id_factory.create();
        let name = Name::new(String::from("John"), String::from("Cooper")).unwrap();
        let email = EmailAddress::new(String::from("demo@test.com")).unwrap();
        let password = String::from("123");
        let result = uc.execute(id.clone(), name, email, password);

        match result {
            Ok(u) => {
                assert_eq!(u.id.to_string(), id.clone().to_string());

                Err("Should return an error")
            }
            Err(e) => match e {
                RegistrationError::SavingError(_) => Ok(()),
                _ => Err("Wrong Error"),
            },
        }
    }
}
