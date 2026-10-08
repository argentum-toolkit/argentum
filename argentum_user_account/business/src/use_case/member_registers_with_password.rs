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

use crate::service::EmailConfirmationInitiator;
use crate::specification::{PasswordSpecificationError, StrongPasswordSpecification};

pub struct MemberRegistersWithPasswordUc<I>
where
    I: Id,
{
    member_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
    credential_writer: Arc<dyn PasswordCredentialWriterTrait<Id = I>>,
    email_confirmation_initiator: Arc<EmailConfirmationInitiator<I>>,
    password_specification: Arc<StrongPasswordSpecification>,
}

impl<I> MemberRegistersWithPasswordUc<I>
where
    I: Id,
    // L: LoggerTrait,
{
    pub fn new(
        member_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
        credential_writer: Arc<dyn PasswordCredentialWriterTrait<Id = I>>,
        email_confirmation_initiator: Arc<EmailConfirmationInitiator<I>>,
        password_specification: Arc<StrongPasswordSpecification>,
    ) -> Self {
        Self {
            member_repository,
            credential_writer,
            email_confirmation_initiator,
            password_specification,
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

        self.password_specification
            .is_satisfied_by(&password)
            .map_err(RegistrationError::PasswordSpecificationErrors)?;

        let result = self.member_repository.find_by_email(&email).await;

        if result?.is_some() {
            return Err(RegistrationError::EmailAlreadyExists);
        }

        // Save user
        let user = {
            let user = Member::new(id, name, email.clone(), false);
            self.member_repository.create(&user).await?;

            user
        };

        let cred = PasswordCredential::new(user.id, password);
        self.credential_writer.write(&cred).await?;

        // Initiate email confirmation
        self.email_confirmation_initiator
            .initiate_confirmation(&user)
            .await
            .map_err(RegistrationError::from)?;

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

    #[error("Password does not meet the strength requirements")]
    PasswordSpecificationErrors(Vec<PasswordSpecificationError>),

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

    #[error(transparent)]
    EmailConfirmationInitiatorError(#[from] crate::service::EmailConfirmationInitiatorError),
}

#[cfg(test)]
mod test {
    use crate::mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock;
    use crate::mock::token::MemberGeneratorMock;
    use crate::repository::password_credential_writer::PasswordCredentialWriter;
    use crate::specification::StrongPasswordSpecification;
    use crate::use_case::member_registers_with_password::{
        MemberRegistersWithPasswordUc, RegistrationError,
    };
    use argentum_encryption_business::mock::password::EncryptorMock;
    use argentum_notification_business::mock::StdoutNotifier;
    use argentum_standard_business::data_type::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::data_type::builder::NameBuilder;
    use argentum_user_business::mock::repository::broken::member_repository_mock::MemberRepositoryMockWihBrokenSave;
    use argentum_user_business::mock::repository::member_repository_mock::MemberRepositoryMock;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_user_registers_with_password() -> Result<(), &'static str> {
        let password_specification = Arc::new(StrongPasswordSpecification::new(3, false));
        let id_factory = Arc::new(IdFactoryMock::new());
        let credential_repository =
            PasswordCredentialRepositoryMock::new(Arc::new(EncryptorMock::new()));
        let credential_writer = PasswordCredentialWriter::new(Arc::new(credential_repository));
        let member_repository = MemberRepositoryMock::new();

        // Mock dependencies for EmailConfirmationInitiator
        use crate::mock::repository::email_confirmation_token_repository_mock::EmailConfirmationTokenRepositoryMock;
        use crate::service::EmailConfirmationInitiator;

        let token_generator = Arc::new(MemberGeneratorMock::new());
        let email_confirmation_token_repository =
            Arc::new(EmailConfirmationTokenRepositoryMock::new());
        let notifier = Arc::new(StdoutNotifier::new());
        let product_name = "TestProduct";
        let email_confirmation_front_url = "http://test.com/confirm/";

        let email_confirmation_initiator = Arc::new(EmailConfirmationInitiator::new(
            token_generator,
            email_confirmation_token_repository,
            notifier,
            product_name,
            email_confirmation_front_url,
        ));

        let uc = MemberRegistersWithPasswordUc::new(
            Arc::new(member_repository),
            Arc::new(credential_writer),
            email_confirmation_initiator,
            password_specification,
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
        let password_specification = Arc::new(StrongPasswordSpecification::new(3, false));
        let id_factory = Arc::new(IdFactoryMock::new());
        let credential_repository =
            PasswordCredentialRepositoryMock::new(Arc::new(EncryptorMock::new()));
        let credential_writer = PasswordCredentialWriter::new(Arc::new(credential_repository));
        let member_repository = MemberRepositoryMock::new();

        // Mock dependencies for EmailConfirmationInitiator
        use crate::mock::repository::email_confirmation_token_repository_mock::EmailConfirmationTokenRepositoryMock;
        use crate::service::EmailConfirmationInitiator;

        let token_generator = Arc::new(MemberGeneratorMock::new());
        let email_confirmation_token_repository =
            Arc::new(EmailConfirmationTokenRepositoryMock::new());
        let notifier = Arc::new(StdoutNotifier::new());
        let product_name = "TestProduct";
        let email_confirmation_front_url = "http://test.com/confirm/";

        let email_confirmation_initiator = Arc::new(EmailConfirmationInitiator::new(
            token_generator,
            email_confirmation_token_repository,
            notifier,
            product_name,
            email_confirmation_front_url,
        ));

        let uc = MemberRegistersWithPasswordUc::new(
            Arc::new(member_repository),
            Arc::new(credential_writer),
            email_confirmation_initiator,
            password_specification,
        );

        let id = id_factory.create();
        let name = NameBuilder::new("John".into())
            .last(Some("Cooper".into()))
            .try_build()
            .expect("Name should be valid");

        let email = EmailAddress::try_new("demo@test.com").expect("Email should be valid");
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
        let password_specification = Arc::new(StrongPasswordSpecification::new(3, false));
        let id_factory = Arc::new(IdFactoryMock::new());
        let credential_repository =
            PasswordCredentialRepositoryMock::new(Arc::new(EncryptorMock::new()));
        let credential_writer = PasswordCredentialWriter::new(Arc::new(credential_repository));
        let member_repository = MemberRepositoryMockWihBrokenSave::new();

        // Mock dependencies for EmailConfirmationInitiator
        use crate::mock::repository::email_confirmation_token_repository_mock::EmailConfirmationTokenRepositoryMock;
        use crate::service::EmailConfirmationInitiator;

        let token_generator = Arc::new(MemberGeneratorMock::new());
        let email_confirmation_token_repository =
            Arc::new(EmailConfirmationTokenRepositoryMock::new());
        let notifier = Arc::new(StdoutNotifier::new());
        let product_name = "TestProduct";
        let email_confirmation_front_url = "http://test.com/confirm/";

        let email_confirmation_initiator = Arc::new(EmailConfirmationInitiator::new(
            token_generator,
            email_confirmation_token_repository,
            notifier,
            product_name,
            email_confirmation_front_url,
        ));

        let uc = MemberRegistersWithPasswordUc::new(
            Arc::new(member_repository),
            Arc::new(credential_writer),
            email_confirmation_initiator,
            password_specification,
        );

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
