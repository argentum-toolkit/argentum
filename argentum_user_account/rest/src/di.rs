use crate::server::handler::GuestRegistersTrait;
use crate::server::handler::GuestRequestsRestoreTokenTrait;
use crate::server::handler::GuestWithTokenChangesPasswordTrait;
use crate::server::handler::MemberLoginsWithPasswordTrait;
use crate::server::handler::MemberRegistersWithPasswordTrait;
use crate::server::handler::MemberVerifiesEmailTrait;

use crate::server::{PreHandler, Router};
use argentum_rest_infrastructure::service::{
    BearerAuthenticator, ErrorPreHandler, RequestTransformer,
};
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::di_container::Container;
use std::sync::Arc;

pub fn user_account_rest_provide<I>(container: &mut Container, url_prefix: &'static str)
where
    I: Id + 'static,
{
    container.provide_factory::<Router<I>, _>(|c| {
        let request_transformer = c.resolve::<RequestTransformer>()?;
        let bearer_auth = c.resolve::<BearerAuthenticator<I>>()?;
        let guest_registers_handler = c.resolve::<dyn GuestRegistersTrait<Id = I>>()?;
        let guest_requests_restore_token_handler =
            c.resolve::<dyn GuestRequestsRestoreTokenTrait<Id = I>>()?;
        let guest_with_token_changes_password_handler =
            c.resolve::<dyn GuestWithTokenChangesPasswordTrait<Id = I>>()?;
        let member_logins_with_password_handler =
            c.resolve::<dyn MemberLoginsWithPasswordTrait<Id = I>>()?;
        let member_registers_with_password_handler =
            c.resolve::<dyn MemberRegistersWithPasswordTrait<Id = I>>()?;
        let member_verifies_email_handler = c.resolve::<dyn MemberVerifiesEmailTrait<Id = I>>()?;
        let error_pre_handler = c.resolve::<ErrorPreHandler>()?;

        let pre_handler = Arc::new(PreHandler::new(
            request_transformer,
            bearer_auth,
            guest_registers_handler,
            guest_requests_restore_token_handler,
            guest_with_token_changes_password_handler,
            member_logins_with_password_handler,
            member_registers_with_password_handler,
            member_verifies_email_handler,
        ));

        Ok(Arc::new(Router::new(
            pre_handler,
            error_pre_handler,
            url_prefix,
        )))
    });
}
