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
use std::sync::Arc;

pub struct ApiDi<I>
where
    I: Id,
{
    pub router: Arc<Router<I>>,
}

impl<I> ApiDi<I>
where
    I: Id,
{
    pub fn new(
        //config
        url_prefix: String,
        //services
        request_transformer: Arc<RequestTransformer>,
        bearer_auth: Arc<BearerAuthenticator<I>>,
        guest_registers_handler: Arc<dyn GuestRegistersTrait<Id = I>>,
        guest_requests_restore_token_handler: Arc<dyn GuestRequestsRestoreTokenTrait<Id = I>>,
        guest_with_token_changes_password_handler: Arc<
            dyn GuestWithTokenChangesPasswordTrait<Id = I>,
        >,
        member_logins_with_password_handler: Arc<dyn MemberLoginsWithPasswordTrait<Id = I>>,
        member_registers_with_password_handler: Arc<dyn MemberRegistersWithPasswordTrait<Id = I>>,
        member_verifies_email_handler: Arc<dyn MemberVerifiesEmailTrait<Id = I>>,
        error_pre_handler: Arc<ErrorPreHandler>,
    ) -> Self {
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

        let router = Arc::new(Router::new(pre_handler, error_pre_handler, url_prefix));

        ApiDi { router }
    }
}
