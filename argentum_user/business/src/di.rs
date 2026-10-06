use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::di_container::Container;

use crate::mock::repository::guest_binding_repository_mock::GuestBindingRepositoryMock;
use crate::mock::repository::guest_repository_mock::GuestRepositoryMock;
use crate::mock::repository::member_repository_mock::MemberRepositoryMock;
use crate::mock::repository::session_repository_mock::SessionRepositoryMock;
use crate::mock::service::TokenRouterMock;
use crate::repository::guest_binding_repository::GuestBindingRepositoryTrait;
use crate::repository::session_repository::SessionRepositoryTrait;
use crate::repository::user_repository::{GuestUserRepositoryTrait, MemberRepositoryTrait};
use crate::service::{GuestBinder, MemberAuthenticatorTrait, TokenRouterTrait};
use crate::use_case::GetUserUc;
use crate::use_case::member_authenticates_with_token::MemberAuthenticatesWithTokenUc;
use std::sync::{Arc, OnceLock};

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

    built_di: OnceLock<Arc<UserBusinessDi<I, L>>>,
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

            built_di: OnceLock::new(),
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

    pub fn build(&self) -> Result<Arc<UserBusinessDi<I, L>>, String> {
        if let Some(di) = self.built_di.get() {
            return Ok(Arc::clone(di));
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

        match self.built_di.set(Arc::clone(&di)) {
            Ok(()) => Ok(di),

            Err(_) => Ok(Arc::clone(
                self.built_di
                    .get()
                    .expect("OnceLock was initialized after set() failed"),
            )),
        }
    }
}

pub fn user_business_provide<I, L>(container: &mut Container)
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    container.provide_factory::<GetUserUc<_>, _>(|c| {
        let member_repository = c.resolve::<dyn MemberRepositoryTrait<Id = I>>()?;

        Ok(Arc::new(GetUserUc::new(member_repository)))
    });

    container.provide_factory::<MemberAuthenticatesWithTokenUc<_>, _>(|c| {
        let member_authenticator = c.resolve::<dyn MemberAuthenticatorTrait<Id = I>>()?;
        let guest_repository = c.resolve::<dyn GuestUserRepositoryTrait<Id = I>>()?;
        let session_repository = c.resolve::<dyn SessionRepositoryTrait<Id = I>>()?;
        let token_router = c.resolve::<dyn TokenRouterTrait>()?;

        Ok(Arc::new(MemberAuthenticatesWithTokenUc::new(
            member_authenticator,
            guest_repository,
            session_repository,
            token_router,
        )))
    });

    container.provide_factory::<GuestBinder<_, _>, _>(|c| {
        let guest_binding_repository = c.resolve::<dyn GuestBindingRepositoryTrait<Id = I>>()?;
        let session_repository = c.resolve::<dyn SessionRepositoryTrait<Id = I>>()?;
        let logger = c.resolve::<L>()?;

        Ok(Arc::new(GuestBinder::new(
            guest_binding_repository,
            session_repository,
            logger,
        )))
    });
}

pub fn user_business_provide_mock<I>(container: &mut Container)
where
    I: Id + 'static,
{
    container.provide_factory::<dyn GuestUserRepositoryTrait<Id = I>, _>(|_c| {
        Ok(Arc::new(GuestRepositoryMock::new()))
    });

    container.provide_factory::<dyn MemberRepositoryTrait<Id = I>, _>(|_c| {
        Ok(Arc::new(MemberRepositoryMock::new()))
    });

    container.provide_factory::<dyn GuestBindingRepositoryTrait<Id = I>, _>(|_c| {
        Ok(Arc::new(GuestBindingRepositoryMock::new()))
    });

    container.provide_factory::<dyn SessionRepositoryTrait<Id = I>, _>(|_c| {
        Ok(Arc::new(SessionRepositoryMock::new()))
    });
    container.provide_factory::<dyn TokenRouterTrait, _>(|_c| Ok(Arc::new(TokenRouterMock::new())));
}
