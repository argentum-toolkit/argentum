use crate::mock::repository::anonymous_binding_repository_mock::AnonymousBindingRepositoryMock;
use crate::mock::repository::anonymous_user_repository_mock::AnonymousUserRepositoryMock;
use crate::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
use crate::repository::anonymous_binding_repository::AnonymousBindingRepositoryTrait;
use crate::repository::user_repository::{
    AnonymousUserRepositoryTrait, AuthenticatedUserRepositoryTrait,
};
use std::sync::Arc;

pub struct UserBusinessDiC {
    pub anonymous_binding_repository: Arc<dyn AnonymousBindingRepositoryTrait>,
    pub anonymous_user_repository: Arc<dyn AnonymousUserRepositoryTrait>,
    pub authenticated_user_repository: Arc<dyn AuthenticatedUserRepositoryTrait>,
}

#[derive(Default)]
pub struct UserBusinessDiCBuilder {
    pub anonymous_binding_repository: Option<Arc<dyn AnonymousBindingRepositoryTrait>>,
    pub anonymous_user_repository: Option<Arc<dyn AnonymousUserRepositoryTrait>>,
    pub authenticated_user_repository: Option<Arc<dyn AuthenticatedUserRepositoryTrait>>,
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

    pub fn build(&self) -> UserBusinessDiC {
        UserBusinessDiC {
            anonymous_binding_repository: self.anonymous_binding_repository.clone().unwrap(),
            anonymous_user_repository: self.anonymous_user_repository.clone().unwrap(),
            authenticated_user_repository: self.authenticated_user_repository.clone().unwrap(),
        }
    }
}
