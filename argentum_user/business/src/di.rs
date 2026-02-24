use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;

use crate::repository::guest_binding_repository::GuestBindingRepositoryTrait;
use crate::repository::session_repository::SessionRepositoryTrait;
use crate::repository::user_repository::{GuestUserRepositoryTrait, MemberRepositoryTrait};
use crate::service::{GuestBinder, MemberAuthenticatorTrait, TokenRouterTrait};
use crate::use_case::GetUserUc;
use crate::use_case::member_authenticates_with_token::MemberAuthenticatesWithTokenUc;
use std::sync::Arc;

pub struct UserBusinessDi<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub guest_repository: Arc<dyn GuestUserRepositoryTrait<Id = I>>,
    pub member_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
    pub user_authenticates_with_token_uc: Arc<MemberAuthenticatesWithTokenUc<I>>,
    pub get_user_uc: Arc<GetUserUc<I>>,
    pub session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
    pub guest_binder: Arc<GuestBinder<I, L>>,
}

#[derive(Default)]
pub struct UserBusinessDiBuilder<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    guest_binding_repository: Option<Arc<dyn GuestBindingRepositoryTrait<Id = I>>>,
    guest_repository: Option<Arc<dyn GuestUserRepositoryTrait<Id = I>>>,
    member_repository: Option<Arc<dyn MemberRepositoryTrait<Id = I>>>,
    session_repository: Option<Arc<dyn SessionRepositoryTrait<Id = I>>>,
    token_router: Option<Arc<dyn TokenRouterTrait>>,
    member_authenticator: Option<Arc<dyn MemberAuthenticatorTrait<Id = I>>>,
    logger: Arc<L>,

    built_di: Option<Arc<UserBusinessDi<I, L>>>,
}

impl<I, L> UserBusinessDiBuilder<I, L>
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    pub fn new(logger: Arc<L>) -> Self {
        Self {
            guest_binding_repository: None,
            guest_repository: None,
            member_repository: None,
            session_repository: None,
            token_router: None,
            member_authenticator: None,
            logger,

            built_di: None,
        }
    }

    pub fn with_guest_binding_repository(
        &mut self,
        repository: Arc<dyn GuestBindingRepositoryTrait<Id = I>>,
    ) -> &mut Self {
        self.guest_binding_repository = Some(repository);

        self
    }

    pub fn with_guest_repository(
        &mut self,
        repository: Arc<dyn GuestUserRepositoryTrait<Id = I>>,
    ) -> &mut Self {
        self.guest_repository = Some(repository);

        self
    }

    pub fn with_member_repository(
        &mut self,
        repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
    ) -> &mut Self {
        self.member_repository = Some(repository);

        self
    }

    pub fn with_session_repository(
        &mut self,
        repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
    ) -> &mut Self {
        self.session_repository = Some(repository);

        self
    }

    pub fn with_token_router(&mut self, router: Arc<dyn TokenRouterTrait>) -> &mut Self {
        self.token_router = Some(router);

        self
    }

    pub fn with_member_authenticator(
        &mut self,
        authenticator: Arc<dyn MemberAuthenticatorTrait<Id = I>>,
    ) -> &mut Self {
        self.member_authenticator = Some(authenticator);

        self
    }

    pub fn build(&mut self) -> Result<Arc<UserBusinessDi<I, L>>, String> {
        if let Some(di) = &self.built_di {
            return Ok(di.clone());
        }

        let member_repository = self
            .member_repository
            .clone()
            .ok_or("member_repository not initialized")?;

        let guest_repository = self
            .guest_repository
            .clone()
            .ok_or("guest_repository not initialized")?;

        let guest_binding_repository = self
            .guest_binding_repository
            .clone()
            .ok_or("guest_binding_repository not initialized")?;

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
            guest_binding_repository.clone(),
            session_repository.clone(),
            self.logger.clone(),
        ));

        let user_authenticates_with_token_uc = Arc::new(MemberAuthenticatesWithTokenUc::new(
            member_authenticator.clone(),
            guest_repository.clone(),
            session_repository.clone(),
            token_router.clone(),
        ));

        let get_user_uc = Arc::new(GetUserUc::new(member_repository.clone()));

        let di = Arc::new(UserBusinessDi {
            guest_repository,
            member_repository,
            user_authenticates_with_token_uc,
            get_user_uc,
            session_repository,
            guest_binder,
        });

        self.built_di = Some(di.clone());

        Ok(di)
    }
}
