use argentum_standard_business::data_type::id::Id;

use crate::mock::repository::anonymous_binding_repository_mock::AnonymousBindingRepositoryMock;
use crate::mock::repository::anonymous_user_repository_mock::AnonymousUserRepositoryMock;
use crate::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
use crate::repository::anonymous_binding_repository::AnonymousBindingRepositoryTrait;
use crate::repository::session_repository::SessionRepositoryTrait;
use crate::repository::user_repository::{
    AnonymousUserRepositoryTrait, AuthenticatedUserRepositoryTrait,
};
use crate::use_case::GetUserUc;
use crate::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use std::sync::Arc;

pub struct UserBusinessDiC<I>
where
    I: Id,
{
    pub anonymous_binding_repository: Arc<dyn AnonymousBindingRepositoryTrait<Id = I>>,
    pub anonymous_user_repository: Arc<dyn AnonymousUserRepositoryTrait<Id = I>>,
    pub authenticated_user_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>,
    pub user_authenticates_with_token_uc: Arc<UserAuthenticatesWithTokenUc<I>>,
    pub get_user_uc: Arc<GetUserUc<I>>,
    pub session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
}

#[derive(Default)]
pub struct UserBusinessDiCBuilder<I>
where
    I: Id,
{
    anonymous_binding_repository: Option<Arc<dyn AnonymousBindingRepositoryTrait<Id = I>>>,
    anonymous_user_repository: Option<Arc<dyn AnonymousUserRepositoryTrait<Id = I>>>,
    authenticated_user_repository: Option<Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>>,
    session_repository: Option<Arc<dyn SessionRepositoryTrait<Id = I>>>,
}

impl<I> UserBusinessDiCBuilder<I>
where
    I: Id + 'static,
{
    pub fn new() -> Self {
        Self {
            anonymous_binding_repository: None,
            anonymous_user_repository: None,
            authenticated_user_repository: None,
            session_repository: None,
        }
    }

    pub fn mock(&mut self) -> &mut Self {
        self.anonymous_user_repository = Some(Arc::new(AnonymousUserRepositoryMock::<I>::new()));

        self.authenticated_user_repository =
            Some(Arc::new(AuthenticatedUserRepositoryMock::<I>::new()));

        self.anonymous_binding_repository =
            Some(Arc::new(AnonymousBindingRepositoryMock::<I>::new()));

        self
    }

    pub fn anonymous_binding_repository(
        &mut self,
        repository: Arc<dyn AnonymousBindingRepositoryTrait<Id = I>>,
    ) -> &mut Self {
        self.anonymous_binding_repository = Some(repository);

        self
    }

    pub fn anonymous_user_repository(
        &mut self,
        repository: Arc<dyn AnonymousUserRepositoryTrait<Id = I>>,
    ) -> &mut Self {
        self.anonymous_user_repository = Some(repository);

        self
    }

    pub fn authenticated_user_repository(
        &mut self,
        repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>,
    ) -> &mut Self {
        self.authenticated_user_repository = Some(repository);

        self
    }

    pub fn session_repository(
        &mut self,
        repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
    ) -> &mut Self {
        self.session_repository = Some(repository);

        self
    }

    pub fn build(&self) -> Result<UserBusinessDiC<I>, String> {
        let authenticated_user_repository = self
            .authenticated_user_repository
            .clone()
            .ok_or("authenticated_user_repository not initialized")?;

        let anonymous_user_repository = self
            .anonymous_user_repository
            .clone()
            .ok_or("anonymous_user_repository not initialized")?;

        let anonymous_binding_repository = self
            .anonymous_binding_repository
            .clone()
            .ok_or("anonymous_binding_repository not initialized")?;

        let session_repository = self
            .session_repository
            .clone()
            .ok_or("session_repository not initialized")?;

        let user_authenticates_with_token_uc = Arc::new(UserAuthenticatesWithTokenUc::new(
            authenticated_user_repository.clone(),
            anonymous_user_repository.clone(),
            session_repository.clone(),
        ));

        let get_user_uc = Arc::new(GetUserUc::new(authenticated_user_repository.clone()));

        Ok(UserBusinessDiC {
            anonymous_binding_repository,
            anonymous_user_repository,
            authenticated_user_repository,
            user_authenticates_with_token_uc,
            get_user_uc,
            session_repository,
        })
    }
}
