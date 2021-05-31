use crate::entity::anonymous_binding::AnonymousBinding;
use crate::entity::session::Session;
use crate::entity::user::{AnonymousUser, UserTrait};
use crate::repository::anonymous_binding_repository::AnonymousBindingRepositoryTrait;
use crate::repository::password_credential_checker::PasswordCredentialChecker;
use crate::repository::session_repository::SessionRepositoryTrait;
use crate::repository::user_repository::AuthenticatedUserRepositoryTrait;
use crate::token::GeneratorTrait;
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::IdFactory;

pub struct UserLoginsWithPasswordUc<'s> {
    user_repository: &'s dyn AuthenticatedUserRepositoryTrait,
    anonymous_binding_repository: &'s dyn AnonymousBindingRepositoryTrait,
    session_repository: &'s dyn SessionRepositoryTrait,
    credential_checker: &'s PasswordCredentialChecker<'s>,
    id_factory: &'s dyn IdFactory,
    token_generator: &'s dyn GeneratorTrait,
}

impl<'s> UserLoginsWithPasswordUc<'s> {
    pub fn new(
        user_repository: &'s dyn AuthenticatedUserRepositoryTrait,
        anonymous_binding_repository: &'s dyn AnonymousBindingRepositoryTrait,
        session_repository: &'s dyn SessionRepositoryTrait,
        credential_checker: &'s PasswordCredentialChecker<'s>,
        id_factory: &'s dyn IdFactory,
        token_generator: &'s dyn GeneratorTrait,
    ) -> UserLoginsWithPasswordUc<'s> {
        UserLoginsWithPasswordUc {
            user_repository,
            anonymous_binding_repository,
            session_repository,
            credential_checker,
            id_factory,
            token_generator,
        }
    }

    pub fn execute(
        &self,
        anonymous: AnonymousUser,
        email: EmailAddress,
        password: String,
    ) -> Result<Session, LoginError> {
        let result = self.user_repository.find_by_email(&email);

        let user = match result {
            Some(u) => u,
            None => return Err(LoginError::WrongEmailOrPassword),
        };

        let ok = self.credential_checker.check(user.id(), &password);

        if !ok {
            return Err(LoginError::WrongEmailOrPassword);
        }

        let session = Session::new(
            self.id_factory.create(),
            user.id().clone(),
            self.token_generator.generate(&user.id),
        );

        let result = match self.session_repository.save(&session) {
            Ok(_) => Result::Ok(session),
            Err(_) => Err(LoginError::SaveSession),
        };

        match self
            .session_repository
            .delete_users_sessions(&anonymous.id())
        {
            Ok(_) => {
                // TODO: log
                println!("INFO: anonymous session deleted")
            }
            Err(_) => {
                // TODO: log error
                println!("WARNING: anonymous session is not deleted")
            }
        };

        let binding = AnonymousBinding::new(user.id(), anonymous.id());
        match self.anonymous_binding_repository.save(&binding) {
            Ok(_) => {
                // TODO: log
                println!("INFO: anonymous binding saved")
            }
            Err(_) => {
                // TODO: log error
                println!("WARNING: anonymous binding is not saved")
            }
        }

        result
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Can't save session")]
    SaveSession,

    #[error("Wrong email or password")]
    WrongEmailOrPassword,
}

#[cfg(test)]
mod test {
    use crate::entity::credential::PasswordCredential;
    use crate::entity::user::{AnonymousUser, AuthenticatedUser};
    use crate::mock::id_factory::IdFactoryMock;
    use crate::mock::repository::anonymous_binding_repository_mock::AnonymousBindingRepositoryMock;
    use crate::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
    use crate::mock::repository::password_credential_repository_mock::PasswordCredentialRepositoryMock;
    use crate::mock::repository::session_repository_mock::SessionRepositoryMock;
    use crate::mock::token::TokenGeneratorMock;
    use crate::repository::anonymous_binding_repository::AnonymousBindingRepositoryTrait;
    use crate::repository::credential_writer::CredentialWriterTrait;
    use crate::repository::password_credential_checker::PasswordCredentialChecker;
    use crate::repository::password_credential_writer::PasswordCredentialWriter;
    use crate::repository::user_repository::AuthenticatedUserRepositoryTrait;
    use crate::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
    use crate::value_object::name::Name;
    use argentum_encryption_business::mock::password::{EncryptorMock, ValidatorMock};
    use argentum_encryption_business::password::Encryptor;
    use argentum_standard_business::data_type::email::EmailAddress;
    use argentum_standard_business::data_type::id::{Id, IdFactory};

    #[test]
    fn test_user_logins_with_passwodr() -> Result<(), &'static str> {
        let user_repository = AuthenticatedUserRepositoryMock::new();
        let anunymous_binding_repository = AnonymousBindingRepositoryMock::new();
        let session_repository = SessionRepositoryMock::new();
        let credendital_repository = PasswordCredentialRepositoryMock::new();
        let validator = ValidatorMock::new();
        let credential_checker =
            PasswordCredentialChecker::new(&credendital_repository, &validator);
        let id_factory = IdFactoryMock::new();
        let token_generator = TokenGeneratorMock::new();
        let credential_writer = PasswordCredentialWriter::new(&credendital_repository);

        let uc = UserLoginsWithPasswordUc::new(
            &user_repository,
            &anunymous_binding_repository,
            &session_repository,
            &credential_checker,
            &id_factory,
            &token_generator,
        );

        let id_factory = IdFactoryMock::new();

        let user_id: Id = id_factory.create();
        let name = Name::new(String::from("Some"), String::from("Name")).unwrap();
        let email = EmailAddress::new(String::from("test@test-mail.com")).unwrap();
        let password = String::from("12345");
        let user = AuthenticatedUser::new(&user_id, name, email.clone());
        let encryptor = EncryptorMock::new();
        let (hashed_password, salt) = encryptor.encrypt(&password).unwrap();
        let cred = PasswordCredential::new(user_id.clone(), hashed_password, salt);

        user_repository.save(&user).expect("Can't save a user");
        credential_writer.write(Box::new(cred));

        let anonymous_id: Id = id_factory.create();
        let anonymous = AnonymousUser::new(&anonymous_id);

        let result = uc.execute(anonymous, email, password);

        match result {
            Ok(s) => {
                assert_eq!(s.user_id.to_string(), user_id.to_string());

                let binding = anunymous_binding_repository
                    .find_by_user_id(&user_id)
                    .unwrap();
                assert_eq!(binding.anonymous_id.to_string(), anonymous_id.to_string());

                return Ok(());
            }
            Err(_) => {
                return Err("User can't login");
            }
        }
    }

    //TODO: negative test
}
