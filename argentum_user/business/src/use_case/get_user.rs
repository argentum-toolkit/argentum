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
    use crate::entity::user::AuthenticatedUser;
    use crate::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
    use crate::repository::user_repository::AuthenticatedUserRepositoryTrait;
    use crate::use_case::get_user::GetUserError;
    use crate::use_case::GetUserUc;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use std::sync::Arc;

    #[test]
    fn test_get_user() -> Result<(), &'static str> {
        let authenticated_user_repository = Arc::new(AuthenticatedUserRepositoryMock::new());
        let id_factory = IdFactoryMock::new();

        //Data
        let user_id: Id = id_factory.create();
        let authenticated_user = AuthenticatedUser::new(
            &user_id,
            NameBuilder::new("Dionne".into())
                .last(Some("Morrison".into()))
                .try_build()
                .unwrap(),
            EmailAddress::try_new("dionne@examples.com".into()).unwrap(),
        );

        //Prefilling
        authenticated_user_repository
            .save(&authenticated_user)
            .expect("Can't save a user");

        //Test
        let uc = GetUserUc::new(authenticated_user_repository);

        let result = uc.execute(user_id.clone());

        match result {
            Ok(u) => {
                assert_eq!(u.id.to_string(), user_id.clone().to_string());
                assert_eq!(u.email.as_string(), "dionne@examples.com");

                return Ok(());
            }
            Err(_) => {
                return Err("Can't get user");
            }
        }
    }

    #[test]
    fn test_get_not_registered_user() -> Result<(), &'static str> {
        let authenticated_user_repository = Arc::new(AuthenticatedUserRepositoryMock::new());
        let id_factory = IdFactoryMock::new();

        //Data
        let user_id: Id = id_factory.create();
        let authenticated_user = AuthenticatedUser::new(
            &user_id,
            NameBuilder::new("Dionne".into())
                .last(Some("Morrison".into()))
                .try_build()
                .unwrap(),
            EmailAddress::try_new("dionne@examples.com".into()).unwrap(),
        );

        //Prefilling
        authenticated_user_repository
            .save(&authenticated_user)
            .expect("Can't save a user");

        //Test
        let uc = GetUserUc::new(authenticated_user_repository);

        let result = uc.execute(id_factory.create());

        match result {
            Err(GetUserError::UserNotFound) => Ok(()),
            Ok(u) => Err("This user is not in storage"),
            Err(_) => Err("Can't get user"),
        }
    }
}
