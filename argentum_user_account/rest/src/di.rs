use crate::server::handler::AnonymousRegistersTrait;
use crate::server::handler::AnonymousRequestsRestoreTokenTrait;
use crate::server::handler::AnonymousWithTokenChangesPasswordTrait;
use crate::server::handler::UserLoginsWithPasswordTrait;
use crate::server::handler::UserRegistersWithPasswordTrait;
use crate::server::{UserAccountPreHandler, UserAccountRouter};
use argentum_rest_infrastructure::service::{
    BearerAuthenticator, ErrorPreHandler, RequestTransformer,
};
use std::sync::Arc;

pub struct ApiDiC {
    pub router: Arc<UserAccountRouter>,
}

impl ApiDiC {
    pub fn new(
        //config
        url_prefix: String,
        //services
        request_transformer: Arc<RequestTransformer>,
        bearer_auth: Arc<BearerAuthenticator>,
        anonymous_registers_handler: Arc<dyn AnonymousRegistersTrait>,
        anonymous_requests_restore_token_handler: Arc<dyn AnonymousRequestsRestoreTokenTrait>,
        anonymous_with_token_changes_password_handler: Arc<
            dyn AnonymousWithTokenChangesPasswordTrait,
        >,
        user_logins_with_password_handler: Arc<dyn UserLoginsWithPasswordTrait>,
        user_registers_with_password_handler: Arc<dyn UserRegistersWithPasswordTrait>,
        error_pre_handler: Arc<ErrorPreHandler>,
    ) -> Self {
        let pre_handler = Arc::new(UserAccountPreHandler::new(
            request_transformer,
            bearer_auth,
            anonymous_registers_handler,
            anonymous_requests_restore_token_handler,
            anonymous_with_token_changes_password_handler,
            user_logins_with_password_handler,
            user_registers_with_password_handler,
        ));

        let router = Arc::new(UserAccountRouter::new(
            pre_handler,
            error_pre_handler,
            url_prefix,
        ));

        ApiDiC { router }
    }
}
