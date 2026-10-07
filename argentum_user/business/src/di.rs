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
use std::sync::Arc;

pub fn user_business_provide<I, L>(container: &mut Container)
where
    I: Id + 'static,
    L: LoggerTrait + 'static,
{
    container.provide_factory::<GetUserUc<_>, _>(|c| {
        let member_repository = c.resolve::<dyn MemberRepositoryTrait<Id = I>>()?;

        Ok(Arc::new(GetUserUc::new(member_repository)))
    });

    container.provide_factory::<MemberAuthenticatesWithTokenUc<I>, _>(|c| {
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
