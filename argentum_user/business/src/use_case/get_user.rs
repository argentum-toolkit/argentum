use crate::entity::user::AuthenticatedUser;
use crate::repository::user_repository::{AuthenticatedUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::id::Id;
use std::sync::Arc;

pub struct GetUserUc {
    user_repository: Arc<dyn AuthenticatedUserRepositoryTrait>,
}

impl GetUserUc {
    pub fn new(user_repository: Arc<dyn AuthenticatedUserRepositoryTrait>) -> Self {
        Self { user_repository }
    }

    pub fn execute(&self, user_id: Id) -> Result<AuthenticatedUser, GetUserError> {
        match self.user_repository.find(&user_id)? {
            Some(u) => Ok(u),
            None => Err(GetUserError::UserNotFound),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetUserError {
    #[error("User not found")]
    UserNotFound,

    #[error("User repository error")]
    UserRepositoryError(
        #[source]
        #[from]
        ExternalUserError,
    ),
}

#[cfg(test)]
mod tests {
    use argentum_standard_business::data_type::email::EmailAddress;
    use argentum_standard_business::data_type::id::{Id, IdFactory};

    use crate::data_type::builder::NameBuilder;
    use crate::entity::session::Session;
    use crate::entity::user::AuthenticatedUser;
    use crate::entity::user::User::{Anonymous, Authenticated};
    use crate::mock::repository::anonymous_user_repository_mock::AnonymousUserRepositoryMock;
    use crate::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
    use crate::mock::repository::session_repository_mock::SessionRepositoryMock;
    use crate::repository::session_repository::SessionRepositoryTrait;
    use crate::repository::user_repository::AuthenticatedUserRepositoryTrait;
    use crate::use_case::user_authenticates_with_token::AuthenticationError;
    use crate::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use std::sync::Arc;

    #[test]
    fn test_authenticates_with_token() -> Result<(), &'static str> {
        let anonymous_user_repository = Arc::new(AnonymousUserRepositoryMock::new());
        let authenticated_user_repository = Arc::new(AuthenticatedUserRepositoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let id_factory = IdFactoryMock::new();

        //Data
        let user_id: Id = id_factory.create();
        let session_id = id_factory.create();
        let token = "test-token".to_string();
        let authenticated_user = AuthenticatedUser::new(
            &user_id,
            NameBuilder::new("Dionne".into())
                .last(Some("Morrison".into()))
                .try_build()
                .unwrap(),
            EmailAddress::try_new("aa@a.com".into()).unwrap(),
        );
        let session = Session::new(session_id, user_id.clone(), token.clone());

        //Prefilling
        authenticated_user_repository
            .save(&authenticated_user)
            .expect("Can't save a user");
        session_repository
            .save(&session)
            .expect("Can't save a session");

        //Test
        let uc = UserAuthenticatesWithTokenUc::new(
            authenticated_user_repository,
            anonymous_user_repository,
            session_repository,
        );

        let result = uc.execute(token.clone());

        match result {
            Ok(u) => match u {
                Authenticated(u) => {
                    assert_eq!(u.id.to_string(), user_id.clone().to_string());

                    return Ok(());
                }
                Anonymous(_) => {
                    return Err("Return's anonymous user, not authenticated");
                }
            },
            Err(_) => {
                return Err("User is not authenticated");
            }
        }
    }

    #[test]
    fn test_authenticates_with_token_should_returns_error_if_token_invalid(
    ) -> Result<(), &'static str> {
        let anonymous_user_repository = Arc::new(AnonymousUserRepositoryMock::new());
        let authenticated_user_repository = Arc::new(AuthenticatedUserRepositoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let id_factory = IdFactoryMock::new();

        //Data
        let user_id: Id = id_factory.create();
        let session_id = id_factory.create();
        let token = "test-token".to_string();
        let authenticated_user = AuthenticatedUser::new(
            &user_id,
            NameBuilder::new("Dionne".into())
                .last(Some("Morrison".into()))
                .try_build()
                .unwrap(),
            EmailAddress::try_new("aa@a.com".into()).unwrap(),
        );
        let session = Session::new(session_id, user_id.clone(), token.clone());

        //Prefilling
        authenticated_user_repository
            .save(&authenticated_user)
            .expect("Can't save a user");
        session_repository
            .save(&session)
            .expect("Can't save a session");

        //Test
        let uc = UserAuthenticatesWithTokenUc::new(
            authenticated_user_repository,
            anonymous_user_repository,
            session_repository,
        );

        let result = uc.execute("wrong-test-token".into());

        match result {
            Ok(_) => {
                return Err("Shpould return error, not an user");
            }
            Err(e) => match e {
                AuthenticationError::WrongToken => return Ok(()),
                _ => {
                    return Err("Invalid response status");
                }
            },
        }
    }

    #[test]
    fn test_authenticates_with_token_should_returns_error_if_user_doesnt_exist(
    ) -> Result<(), &'static str> {
        let anonymous_user_repository = Arc::new(AnonymousUserRepositoryMock::new());
        let authenticated_user_repository = Arc::new(AuthenticatedUserRepositoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let id_factory = IdFactoryMock::new();

        //Data
        let user_id: Id = id_factory.create();
        let session_id = id_factory.create();
        let token = "test-token".to_string();
        let session = Session::new(session_id, user_id.clone(), token.clone());

        //Prefilling
        session_repository
            .save(&session)
            .expect("Can't save a session");

        //Test
        let uc = UserAuthenticatesWithTokenUc::new(
            authenticated_user_repository,
            anonymous_user_repository,
            session_repository,
        );

        let result = uc.execute(token.clone());

        match result {
            Ok(_) => {
                return Err("Shpould return error, not an user");
            }
            Err(e) => match e {
                AuthenticationError::UserNotFound => return Ok(()),
                _ => {
                    return Err("Invalid response status");
                }
            },
        }
    }
}
