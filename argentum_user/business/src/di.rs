use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;

use crate::repository::anonymous_binding_repository::AnonymousBindingRepositoryTrait;
use crate::repository::session_repository::SessionRepositoryTrait;
use crate::repository::user_repository::{
    AnonymousUserRepositoryTrait, AuthenticatedUserRepositoryTrait,
};
use crate::service::{GuestBinder, MemberAuthenticatorTrait, TokenRouterTrait};
use crate::use_case::GetUserUc;
use crate::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use std::sync::Arc;

pub struct UserBusinessDi<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub anonymous_user_repository: Arc<dyn AnonymousUserRepositoryTrait<Id = I>>,
    pub authenticated_user_repository: Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>,
    pub user_authenticates_with_token_uc: Arc<UserAuthenticatesWithTokenUc<I>>,
    pub get_user_uc: Arc<GetUserUc<I>>,
    pub session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
    pub token_router: Arc<dyn TokenRouterTrait>,
    pub guest_binder: Arc<GuestBinder<I, L>>,
}

#[derive(Default)]
pub struct UserBusinessDiBuilder<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    anonymous_binding_repository: Option<Arc<dyn AnonymousBindingRepositoryTrait<Id = I>>>,
    anonymous_user_repository: Option<Arc<dyn AnonymousUserRepositoryTrait<Id = I>>>,
    authenticated_user_repository: Option<Arc<dyn AuthenticatedUserRepositoryTrait<Id = I>>>,
    session_repository: Option<Arc<dyn SessionRepositoryTrait<Id = I>>>,
    token_router: Option<Arc<dyn TokenRouterTrait>>,
    member_authenticator: Option<Arc<dyn MemberAuthenticatorTrait<Id = I>>>,
    logger: Arc<L>,
}

impl<I, L> UserBusinessDiBuilder<I, L>
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    pub fn new(logger: Arc<L>) -> Self {
        Self {
            anonymous_binding_repository: None,
            anonymous_user_repository: None,
            authenticated_user_repository: None,
            session_repository: None,
            token_router: None,
            member_authenticator: None,
            logger,
        }
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

    pub fn token_router(&mut self, router: Arc<dyn TokenRouterTrait>) -> &mut Self {
        self.token_router = Some(router);

        self
    }

    pub fn member_authenticator(
        &mut self,
        authenticator: Arc<dyn MemberAuthenticatorTrait<Id = I>>,
    ) -> &mut Self {
        self.member_authenticator = Some(authenticator);

        self
    }

    pub fn build(&self) -> Result<UserBusinessDi<I, L>, String> {
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

        let token_router = self
            .token_router
            .clone()
            .ok_or("token_router not initialized")?;

        let member_authenticator = self
            .member_authenticator
            .clone()
            .ok_or("member_authenticator not initialized")?;

        let guest_binder = Arc::new(GuestBinder::new(
            anonymous_binding_repository.clone(),
            session_repository.clone(),
            self.logger.clone(),
        ));

        let user_authenticates_with_token_uc = Arc::new(UserAuthenticatesWithTokenUc::new(
            member_authenticator.clone(),
            anonymous_user_repository.clone(),
            session_repository.clone(),
            token_router.clone(),
        ));

        let get_user_uc = Arc::new(GetUserUc::new(authenticated_user_repository.clone()));

        Ok(UserBusinessDi {
            anonymous_user_repository,
            authenticated_user_repository,
            user_authenticates_with_token_uc,
            get_user_uc,
            session_repository,
            token_router,
            guest_binder,
        })
    }
}
