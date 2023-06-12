use crate::entity::session::Session;
use crate::repository::session_repository::{SessionRepositoryError, SessionRepositoryTrait};
use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_user_business::entity::user::{AnonymousUser, UserTrait};
use argentum_user_business::repository::user_repository::{
    AnonymousUserRepositoryTrait, ExternalUserError,
};

use crate::token::GeneratorTrait;
use std::sync::Arc;

pub struct AnonymousRegistersUc {
    id_factory: Arc<dyn IdFactory>,
    user_repository: Arc<dyn AnonymousUserRepositoryTrait>,
    session_repository: Arc<dyn SessionRepositoryTrait>,
    token_generator: Arc<dyn GeneratorTrait>,
}

impl AnonymousRegistersUc {
    pub fn new(
        id_factory: Arc<dyn IdFactory>,
        user_repository: Arc<dyn AnonymousUserRepositoryTrait>,
        session_repository: Arc<dyn SessionRepositoryTrait>,
        token_generator: Arc<dyn GeneratorTrait>,
    ) -> Self {
        Self {
            id_factory,
            user_repository,
            session_repository,
            token_generator,
        }
    }

    pub fn execute(&self, id: &Id) -> Result<(AnonymousUser, Session), AnonymousRegistrationError> {
        let user = {
            let user = AnonymousUser::new(id);

            self.user_repository.save(&user)?;

            user
        };

        let session = Session::new(
            self.id_factory.create(),
            user.id().clone(),
            self.token_generator.generate(id),
        );

        self.session_repository.save(&session)?;

        Ok((user, session))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AnonymousRegistrationError {
    #[error("Can't save anonymous")]
    SavingAnonymousError(
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
    use crate::mock::repository::broken::session_repository_mock::SessionRepositoryMockWithBrokenSave;
    use crate::mock::repository::session_repository_mock::SessionRepositoryMock;
    use crate::mock::token::TokenGeneratorMock;
    use crate::use_case::anonymous_registers::{AnonymousRegistersUc, AnonymousRegistrationError};
    use argentum_standard_business::data_type::id::{Id, IdFactory};
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::mock::repository::anonymous_user_repository_mock::AnonymousUserRepositoryMock;
    use argentum_user_business::mock::repository::broken::anonymous_user_repository_mock::AnonymousRepositoryMockWithBrokenSave;
    use std::sync::Arc;

    #[test]
    fn anonymous_registers() -> Result<(), &'static str> {
        let anonymous_user_repository = Arc::new(AnonymousUserRepositoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_generator = Arc::new(TokenGeneratorMock::new());

        let uc = AnonymousRegistersUc::new(
            id_factory.clone(),
            anonymous_user_repository,
            session_repository,
            token_generator,
        );

        let anon_id: Id = id_factory.create();
        let result = uc.execute(&anon_id);

        match result {
            Ok((anonymous, s)) => {
                assert_eq!(anonymous.id.to_string(), anon_id.clone().to_string());
                assert_eq!(s.user_id.to_string(), anon_id.clone().to_string());

                return Ok(());
            }
            Err(_) => {
                return Err("User is not registered");
            }
        }
    }

    #[test]
    fn anonymous_registers_with_broken_user_repository() -> Result<(), &'static str> {
        let anonymous_user_repository = Arc::new(AnonymousRepositoryMockWithBrokenSave::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_generator = Arc::new(TokenGeneratorMock::new());

        let uc = AnonymousRegistersUc::new(
            id_factory.clone(),
            anonymous_user_repository.clone(),
            session_repository.clone(),
            token_generator.clone(),
        );

        let anon_id: Id = id_factory.create();
        let result = uc.execute(&anon_id);

        match result {
            Ok(_) => Err("Should return error"),
            Err(e) => match e {
                AnonymousRegistrationError::SavingAnonymousError(_) => Ok(()),
                _ => Err("Wrong error type"),
            },
        }
    }
    #[test]
    fn anonymous_registers_with_broken_session_repository() -> Result<(), &'static str> {
        let anonymous_user_repository = Arc::new(AnonymousUserRepositoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMockWithBrokenSave::new());
        let id_factory = Arc::new(IdFactoryMock::new());
        let token_generator = Arc::new(TokenGeneratorMock::new());

        let uc = AnonymousRegistersUc::new(
            id_factory.clone(),
            anonymous_user_repository.clone(),
            session_repository.clone(),
            token_generator.clone(),
        );

        let anon_id: Id = id_factory.create();
        let result = uc.execute(&anon_id);

        match result {
            Ok(_) => Err("Should return error"),
            Err(e) => match e {
                AnonymousRegistrationError::SavingSessionError(_) => Ok(()),
                _ => Err("Wrong error type"),
            },
        }
    }
}
