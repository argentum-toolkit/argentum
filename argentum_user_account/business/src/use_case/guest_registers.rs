use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_user_business::entity::session::Session;
use argentum_user_business::entity::user::{Guest, UserTrait};
use argentum_user_business::repository::session_repository::{
    SessionRepositoryError, SessionRepositoryTrait,
};
use argentum_user_business::repository::user_repository::{
    ExternalUserError, GuestUserRepositoryTrait,
};

use std::sync::Arc;

use crate::token::GuestGeneratorTrait;

pub struct GuestRegistersUc<I>
where
    I: Id,
{
    id_factory: Arc<dyn IdFactory<Id = I>>,
    user_repository: Arc<dyn GuestUserRepositoryTrait<Id = I>>,
    session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
    token_generator: Arc<dyn GuestGeneratorTrait<Id = I>>,
}

impl<I> GuestRegistersUc<I>
where
    I: Id,
{
    pub fn new(
        id_factory: Arc<dyn IdFactory<Id = I>>,
        user_repository: Arc<dyn GuestUserRepositoryTrait<Id = I>>,
        session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
        token_generator: Arc<dyn GuestGeneratorTrait<Id = I>>,
    ) -> Self {
        Self {
            id_factory,
            user_repository,
            session_repository,
            token_generator,
        }
    }

    pub async fn execute(&self, id: I) -> Result<(Guest<I>, Session<I>), GuestRegistrationError> {
        let user = {
            let user = Guest::new(id);

            self.user_repository.save(&user).await?;

            user
        };

        let session = Session::new(
            self.id_factory.create(),
            user.id(),
            &self.token_generator.generate(id),
        );

        self.session_repository.save(&session).await?;

        Ok((user, session))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GuestRegistrationError {
    #[error("Can't save the guest")]
    SavingGuestError(
        #[from]
        #[source]
        ExternalUserError,
    ),

    #[error("Can't save session")]
    SavingSessionError(
        #[from]
        #[source]
        SessionRepositoryError,
    ),
}

#[cfg(test)]
mod tests {
    use crate::mock::token::GuestGeneratorMock;
    use crate::use_case::guest_registers::{GuestRegistersUc, GuestRegistrationError};
    use argentum_standard_business::data_type::id::IdFactory;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::mock::repository::broken::guest_repository_mock::GuestRepositoryMockWithBrokenSave;
    use argentum_user_business::mock::repository::broken::session_repository_mock::SessionRepositoryMockWithBrokenSave;
    use argentum_user_business::mock::repository::guest_repository_mock::GuestRepositoryMock;
    use argentum_user_business::mock::repository::session_repository_mock::SessionRepositoryMock;
    use std::sync::Arc;

    #[tokio::test]
    async fn guest_registers() -> Result<(), &'static str> {
        let guest_repository = Arc::new(GuestRepositoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_generator = Arc::new(GuestGeneratorMock::new());

        let uc = GuestRegistersUc::new(
            id_factory.clone(),
            guest_repository,
            session_repository,
            token_generator,
        );

        let guest_id = id_factory.create();
        let result = uc.execute(guest_id).await;

        match result {
            Ok((guest, s)) => {
                assert_eq!(guest.id.to_string(), guest_id.clone().to_string());
                assert_eq!(s.user_id.to_string(), guest_id.clone().to_string());

                return Ok(());
            }
            Err(_) => {
                return Err("User is not registered");
            }
        }
    }

    #[tokio::test]
    async fn guest_registers_with_broken_user_repository() -> Result<(), &'static str> {
        let guest_repository = Arc::new(GuestRepositoryMockWithBrokenSave::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_generator = Arc::new(GuestGeneratorMock::new());

        let uc = GuestRegistersUc::new(
            id_factory.clone(),
            guest_repository.clone(),
            session_repository.clone(),
            token_generator.clone(),
        );

        let guest_id = id_factory.create();
        let result = uc.execute(guest_id).await;

        match result {
            Ok(_) => Err("Should return error"),
            Err(e) => match e {
                GuestRegistrationError::SavingGuestError(_) => Ok(()),
                _ => Err("Wrong error type"),
            },
        }
    }
    #[tokio::test]
    async fn guest_registers_with_broken_session_repository() -> Result<(), &'static str> {
        let guest_repository = Arc::new(GuestRepositoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMockWithBrokenSave::new());
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_generator = Arc::new(GuestGeneratorMock::new());

        let uc = GuestRegistersUc::new(
            id_factory.clone(),
            guest_repository.clone(),
            session_repository.clone(),
            token_generator.clone(),
        );

        let guest_id = id_factory.create();
        let result = uc.execute(guest_id).await;

        match result {
            Ok(_) => Err("Should return error"),
            Err(e) => match e {
                GuestRegistrationError::SavingSessionError(_) => Ok(()),
                _ => Err("Wrong error type"),
            },
        }
    }
}
