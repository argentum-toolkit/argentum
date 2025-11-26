use crate::entity::user::AuthenticatedUser;
use crate::repository::user_repository::{AuthenticatedUserRepositoryTrait, ExternalUserError};
use argentum_standard_business::data_type::id::Id;
use std::sync::Arc;

pub struct GetUserUc<I>
where
    I: Id,
{
    user_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>,
}

impl<I> GetUserUc<I>
where
    I: Id,
{
    pub fn new(user_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, user_id: I) -> Result<AuthenticatedUser<I>, GetUserError> {
        match self.user_repository.find(user_id).await? {
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
    use argentum_standard_business::data_type::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;

    use crate::data_type::builder::NameBuilder;
    use crate::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
    use crate::repository::user_repository::AuthenticatedUserRepositoryTrait;
    use crate::use_case::GetUserUc;
    use crate::use_case::get_user::GetUserError;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_get_user() -> Result<(), String> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let authenticated_user_repository =
            Arc::new(AuthenticatedUserRepositoryMock::new(id_factory));

        //Data
        let name = NameBuilder::new("Dionne")
            .last(Some("Morrison"))
            .try_build()
            .expect("Name should be valid");

        let email = EmailAddress::try_new("dionne@examples.com").expect("Email should be valid");

        //Prefilling
        let authenticated_user = authenticated_user_repository
            .create(&name, &email)
            .await
            .expect("Can't save a user");

        let user_id = authenticated_user.id;
        //Test
        let uc = GetUserUc::new(authenticated_user_repository);

        let result = uc.execute(user_id).await;
        match result {
            Ok(u) => {
                assert_eq!(u.id.to_string(), user_id.to_string());
                assert_eq!(u.email.as_string(), "dionne@examples.com");

                return Ok(());
            }
            Err(e) => {
                return Err(format!("Can't get user. Error: {}", e));
            }
        }
    }

    #[tokio::test]
    async fn test_get_not_registered_user() -> Result<(), &'static str> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let authenticated_user_repository =
            Arc::new(AuthenticatedUserRepositoryMock::new(id_factory.clone()));

        //Data
        let name = NameBuilder::new("Dionne")
            .last(Some("Morrison"))
            .try_build()
            .expect("Name should be valid");

        let email = EmailAddress::try_new("dionne@examples.com").expect("Email should be valid");

        //Prefilling
        authenticated_user_repository
            .create(&name, &email)
            .await
            .expect("Can't save a user");

        //Test
        let uc = GetUserUc::new(authenticated_user_repository);

        let result = uc.execute(id_factory.create()).await;

        match result {
            Err(GetUserError::UserNotFound) => Ok(()),
            Ok(_) => Err("This user is not in storage"),
            Err(_) => Err("Can't get user"),
        }
    }
}
