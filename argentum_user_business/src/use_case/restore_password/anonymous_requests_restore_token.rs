use crate::entity::restore_password_token::RestorePasswordToken;
use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
use crate::repository::user_repository::AuthenticatedUserRepositoryTrait;
use crate::token::GeneratorTrait;
use crate::use_case::restore_password::error::RestorePasswordError;
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::IdFactory;

pub struct AnonymousRequestsRestoreToken<'s> {
    id_factory: &'s dyn IdFactory,
    user_repository: &'s dyn AuthenticatedUserRepositoryTrait,
    restore_password_token_repository: &'s dyn RestorePasswordTokenRepositoryTrait,
    token_generator: &'s dyn GeneratorTrait,
}

impl<'s> AnonymousRequestsRestoreToken<'s> {
    pub fn new(
        id_factory: &'s dyn IdFactory,
        user_repository: &'s dyn AuthenticatedUserRepositoryTrait,
        restore_password_token_repository: &'s dyn RestorePasswordTokenRepositoryTrait,
        token_generator: &'s dyn GeneratorTrait,
    ) -> AnonymousRequestsRestoreToken<'s> {
        AnonymousRequestsRestoreToken {
            id_factory,
            user_repository,
            restore_password_token_repository,
            token_generator,
        }
    }

    pub fn execute(
        &self,
        email: EmailAddress,
    ) -> Result<RestorePasswordToken, RestorePasswordError> {
        let result = self.user_repository.find_by_email(&email);
        let user = match result {
            Err(err) => return Err(RestorePasswordError::GetUserError(err)),
            Ok(o) => match o {
                Some(user) => user,
                None => return Err(RestorePasswordError::UserNotFoundError),
            },
        };

        let token = self.token_generator.generate(&user.id);
        let id = self.id_factory.create();

        let restore_token = RestorePasswordToken::new(id, user.id.clone(), token);

        if let Err(e) = self
            .restore_password_token_repository
            .delete_users_tokens(&user.id)
        {
            return Err(RestorePasswordError::TokenRepositoryError(e));
        }

        if let Err(e) = self.restore_password_token_repository.save(&restore_token) {
            return Err(RestorePasswordError::TokenRepositoryError(e));
        }

        //TODO: send email

        Ok(restore_token)
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::user::AuthenticatedUser;
    use crate::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
    use crate::mock::repository::restore_password_token_repository_mock::RestorePasswordTokenRepositoryMock;
    use crate::mock::token::TokenGeneratorMock;
    use crate::repository::restore_password_token_repository::RestorePasswordTokenRepositoryTrait;
    use crate::repository::user_repository::AuthenticatedUserRepositoryTrait;
    use crate::use_case::restore_password::anonymous_requests_restore_token::{
        AnonymousRequestsRestoreToken, RestorePasswordError,
    };
    use crate::value_object::name::Name;
    use argentum_standard_business::data_type::email::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;

    #[test]
    fn anonymous_requests_restore_token() -> Result<(), &'static str> {
        let id_factory = IdFactoryMock::new();
        let token_repository = RestorePasswordTokenRepositoryMock::new();
        let user_repository = AuthenticatedUserRepositoryMock::new();
        let token_generator = TokenGeneratorMock::new();

        let uc = AnonymousRequestsRestoreToken::new(
            &id_factory,
            &user_repository,
            &token_repository,
            &token_generator,
        );

        let user_id = id_factory.create();
        let user_name = Name::new("Dionne".to_string(), "Morrison".to_string()).unwrap();
        let email = EmailAddress::new("test@mail.com".to_string()).unwrap();

        let user = AuthenticatedUser::new(&user_id, user_name, email.clone());

        user_repository.save(&user).unwrap();

        let result = uc.execute(email);

        if let Err(_) = result {
            return Err("User is not registered");
        }

        let token = result.unwrap();

        assert!(user_id.eq(&token.user_id), "Wrong user id in token");

        match token_repository.find_by_token(token.token).unwrap() {
            Some(stored_token) => {
                assert!(stored_token.id.eq(&token.id), "Wrong token id");
                assert!(user_id.eq(&stored_token.user_id), "Wrong user id in token");

                Ok(())
            }
            None => Err("Token is not saved"),
        }
    }

    #[test]
    fn anonymous_requests_restore_token_for_not_registered_email() -> Result<(), &'static str> {
        let id_factory = IdFactoryMock::new();
        let token_repository = RestorePasswordTokenRepositoryMock::new();
        let user_repository = AuthenticatedUserRepositoryMock::new();
        let token_generator = TokenGeneratorMock::new();

        let uc = AnonymousRequestsRestoreToken::new(
            &id_factory,
            &user_repository,
            &token_repository,
            &token_generator,
        );

        let email = EmailAddress::new("test@mail.com".to_string()).unwrap();

        let result = uc.execute(email);

        match result {
            Err(e) => match e {
                RestorePasswordError::UserNotFoundError => Ok(()),
                _ => Err("Wrong error type"),
            },
            Ok(_) => Err("Should return not found error"),
        }
    }
}
