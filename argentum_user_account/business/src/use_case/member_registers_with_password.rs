use crate::entity::credential::PasswordCredential;
use crate::repository::password_credential_writer::PasswordCredentialWriterTrait;

use argentum_encryption_business::password::EncryptionError;
use argentum_standard_business::data_type::EmailAddress;
use argentum_user_business::data_type::Name;
use argentum_user_business::entity::user::Member;
use argentum_user_business::repository::user_repository::{
    ExternalUserError, MemberRepositoryTrait,
};
use secrecy::SecretString;

use crate::repository::credential_writer::CredentialWriterError;
use argentum_standard_business::data_type::id::Id;
use std::sync::Arc;

pub struct MemberRegistersWithPasswordUc<I>
where
    I: Id,
{
    user_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
    credential_writer: Arc<dyn PasswordCredentialWriterTrait<Id = I>>,
}

impl<I> MemberRegistersWithPasswordUc<I>
where
    I: Id,
{
    pub fn new(
        user_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
        credential_writer: Arc<dyn PasswordCredentialWriterTrait<Id = I>>,
    ) -> Self {
        Self {
            user_repository,
            credential_writer,
        }
    }

    pub async fn execute(
        &self,
        id: I,
        name: Name,
        email: EmailAddress,
        password: SecretString,
        terms_accepted: bool,
    ) -> Result<Member<I>, RegistrationError> {
        if !terms_accepted {
            return Err(RegistrationError::TermsNotAccepted);
        }

        let result = self.user_repository.find_by_email(&email).await;

        if result?.is_some() {
            return Err(RegistrationError::EmailAlreadyExists);
        }

        //save user
        let user = {
            //it is `temporary mutability` pattern
            let user = Member::new(id, name, email);
            self.user_repository.create(&user).await?;

            user
        };

        //save credentials
        // let (hashed_password, salt) = self.encryptor.encrypt(password)?;

        // let cred = PasswordCredential::new(user.id, &hashed_password, &salt);

        let cred = PasswordCredential::new(user.id, password);
        self.credential_writer.write(&cred).await?;

        Ok(user)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RegistrationError {
    #[error(
        "Please confirm that you agree to the Terms and Conditions and our Privacy Policy to proceed."
    )]
    TermsNotAccepted,

    #[error("User with such email already exists")]
    EmailAlreadyExists,

    //TODO: there is no such error. map it
    #[error("Can't encrypt password")]
    EncryptionError(
        #[source]
        #[from]
        EncryptionError,
    ),

    #[error("External user's error")]
    SavingUserError(
        #[from]
        #[source]
        ExternalUserError,
    ),

    #[error("External user's error")]
    SavingCredentialsError(
        #[source]
        #[from]
        CredentialWriterError,
    ),
}

#[cfg(test)]
mod test {
    use crate::mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock;
    use crate::repository::password_credential_writer::PasswordCredentialWriter;
    use crate::use_case::member_registers_with_password::{
        MemberRegistersWithPasswordUc, RegistrationError,
    };
    use argentum_encryption_business::mock::password::EncryptorMock;
    use argentum_standard_business::data_type::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::data_type::builder::NameBuilder;
    use argentum_user_business::mock::repository::broken::member_repository_mock::MemberRepositoryMockWihBrokenSave;
    use argentum_user_business::mock::repository::member_repository_mock::MemberRepositoryMock;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_user_registers_with_password() -> Result<(), &'static str> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let credential_repository =
            PasswordCredentialRepositoryMock::new(Arc::new(EncryptorMock::new()));
        let credential_writer = PasswordCredentialWriter::new(Arc::new(credential_repository));
        let member_repository = MemberRepositoryMock::new();
        let uc = MemberRegistersWithPasswordUc::new(
            Arc::new(member_repository),
            Arc::new(credential_writer),
        );

        let id = id_factory.create();
        let name = NameBuilder::new("John")
            .last(Some("Cooper"))
            .try_build()
            .expect("Name should be valid");
        let email = EmailAddress::try_new("demo@test.com").expect("Email should be valid");
        let password = "123";
        let terms = true;
        let result = uc.execute(id, name, email, password.into(), terms).await;

        match result {
            Ok(u) => {
                assert_eq!(u.id, id);

                return Ok(());
            }
            Err(_) => {
                return Err("Can't register an user");
            }
        }
    }

    #[tokio::test]
    async fn test_user_registers_without_accepting_of_terms() -> Result<(), &'static str> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let credential_repository =
            PasswordCredentialRepositoryMock::new(Arc::new(EncryptorMock::new()));
        let credential_writer = PasswordCredentialWriter::new(Arc::new(credential_repository));
        let member_repository = MemberRepositoryMock::new();
        let uc = MemberRegistersWithPasswordUc::new(
            Arc::new(member_repository),
            Arc::new(credential_writer),
        );

        let id = id_factory.create();
        let name = NameBuilder::new("John".into())
            .last(Some("Cooper".into()))
            .try_build()
            .expect("Name should be valid");
        let email = EmailAddress::try_new("demo@test.com".into()).expect("Email should be valid");
        let password = "123";
        let terms = false;
        let result = uc.execute(id, name, email, password.into(), terms).await;

        match result {
            Ok(u) => {
                assert_eq!(u.id.to_string(), id.clone().to_string());

                Err("Should return an error")
            }
            Err(e) => match e {
                RegistrationError::TermsNotAccepted => Ok(()),
                _ => Err("Wrong Error"),
            },
        }
    }

    #[tokio::test]
    async fn test_user_registers_with_password_with_broken_user_repository()
    -> Result<(), &'static str> {
        let credential_repository =
            PasswordCredentialRepositoryMock::new(Arc::new(EncryptorMock::new()));
        let credential_writer = PasswordCredentialWriter::new(Arc::new(credential_repository));
        let member_repository = MemberRepositoryMockWihBrokenSave::new();
        let uc = MemberRegistersWithPasswordUc::new(
            Arc::new(member_repository),
            Arc::new(credential_writer),
        );

        let id_factory = IdFactoryMock::new();

        let id = id_factory.create();
        let name = NameBuilder::new("John".into())
            .last(Some("Cooper".into()))
            .try_build()
            .expect("Name should be valid");
        let email = EmailAddress::try_new("demo@test.com".into()).expect("Email should be valid");
        let password = "123";
        let terms = true;
        let result = uc.execute(id, name, email, password.into(), terms).await;

        match result {
            Ok(u) => {
                assert_eq!(u.id.to_string(), id.clone().to_string());

                Err("Should return an error")
            }
            Err(e) => match e {
                RegistrationError::SavingUserError(_) => Ok(()),
                _ => Err("Wrong Error"),
            },
        }
    }
}
