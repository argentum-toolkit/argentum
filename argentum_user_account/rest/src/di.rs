use crate::server::handler::AnonymousRegistersTrait;
use crate::server::handler::AnonymousRequestsRestoreTokenTrait;
use crate::server::handler::AnonymousWithTokenChangesPasswordTrait;
use crate::server::handler::UserLoginsWithPasswordTrait;
use crate::server::handler::UserRegistersWithPasswordTrait;
use crate::server::{PreHandler, Router};
use argentum_rest_infrastructure::service::{
    BearerAuthenticator, ErrorPreHandler, RequestTransformer,
};
use argentum_standard_business::data_type::id::Id;
use std::sync::Arc;

pub struct ApiDiC<I>
where
    I: Id,
{
    pub router: Arc<Router<I>>,
}

impl<I> ApiDiC<I>
where
    I: Id,
{
    pub fn new(
        //config
        url_prefix: String,
        //services
        request_transformer: Arc<RequestTransformer>,
        bearer_auth: Arc<BearerAuthenticator<I>>,
        anonymous_registers_handler: Arc<dyn AnonymousRegistersTrait<Id = I>>,
        anonymous_requests_restore_token_handler: Arc<
            dyn AnonymousRequestsRestoreTokenTrait<Id = I>,
        >,
        anonymous_with_token_changes_password_handler: Arc<
            dyn AnonymousWithTokenChangesPasswordTrait<Id = I>,
        >,
        user_logins_with_password_handler: Arc<dyn UserLoginsWithPasswordTrait<Id = I>>,
        user_registers_with_password_handler: Arc<dyn UserRegistersWithPasswordTrait<Id = I>>,
        error_pre_handler: Arc<ErrorPreHandler>,
    ) -> Self {
        let pre_handler = Arc::new(PreHandler::new(
            request_transformer,
            bearer_auth,
            anonymous_registers_handler,
            anonymous_requests_restore_token_handler,
            anonymous_with_token_changes_password_handler,
            user_logins_with_password_handler,
            user_registers_with_password_handler,
        ));

        let router = Arc::new(Router::new(pre_handler, error_pre_handler, url_prefix));

        ApiDiC { router }
    }
}
