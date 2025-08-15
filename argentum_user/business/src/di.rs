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

pub struct UserBusinessDiC {
    pub anonymous_binding_repository: Arc<dyn AnonymousBindingRepositoryTrait>,
    pub anonymous_user_repository: Arc<dyn AnonymousUserRepositoryTrait>,
    pub authenticated_user_repository: Arc<dyn AuthenticatedUserRepositoryTrait>,
    pub user_authenticates_with_token_uc: Arc<UserAuthenticatesWithTokenUc>,
    pub get_user_uc: Arc<GetUserUc>,
    pub session_repository: Arc<dyn SessionRepositoryTrait>,
}

#[derive(Default)]
pub struct UserBusinessDiCBuilder {
    anonymous_binding_repository: Option<Arc<dyn AnonymousBindingRepositoryTrait>>,
    anonymous_user_repository: Option<Arc<dyn AnonymousUserRepositoryTrait>>,
    authenticated_user_repository: Option<Arc<dyn AuthenticatedUserRepositoryTrait>>,
    session_repository: Option<Arc<dyn SessionRepositoryTrait>>,
}

impl UserBusinessDiCBuilder {
    pub fn mock(&mut self) -> &mut Self {
        self.anonymous_user_repository = Some(Arc::new(AnonymousUserRepositoryMock::new()));

        self.authenticated_user_repository = Some(Arc::new(AuthenticatedUserRepositoryMock::new()));

        self.anonymous_binding_repository = Some(Arc::new(AnonymousBindingRepositoryMock::new()));

        self
    }

    pub fn anonymous_binding_repository(
        &mut self,
        repository: Arc<dyn AnonymousBindingRepositoryTrait>,
    ) -> &mut Self {
        self.anonymous_binding_repository = Some(repository);

        self
    }

    pub fn anonymous_user_repository(
        &mut self,
        repository: Arc<dyn AnonymousUserRepositoryTrait>,
    ) -> &mut Self {
        self.anonymous_user_repository = Some(repository);

        self
    }

    pub fn authenticated_user_repository(
        &mut self,
        repository: Arc<dyn AuthenticatedUserRepositoryTrait>,
    ) -> &mut Self {
        self.authenticated_user_repository = Some(repository);

        self
    }

    pub fn session_repository(&mut self, repository: Arc<dyn SessionRepositoryTrait>) -> &mut Self {
        self.session_repository = Some(repository);

        self
    }

    pub fn build(&self) -> Result<UserBusinessDiC, String> {
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
