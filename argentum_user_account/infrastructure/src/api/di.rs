use crate::api::server::handler::{
    AnonymousRegistersTrait, AnonymousRequestsRestoreTokenTrait,
    AnonymousWithTokenChangesPasswordTrait, UserLoginsWithPasswordTrait,
    UserRegistersWithPasswordTrait,
};
use crate::api::server::{UserAccountPreHandler, UserAccountRouter};
use argentum_rest_infrastructure::service::{
    BearerAuthenticator, ErrorPreHandler, RequestTransformer,
};
use std::sync::Arc;

pub struct ApiDiC {
    pub router: Arc<UserAccountRouter>,
}

impl ApiDiC {
    pub fn new(
        request_transformer: Arc<RequestTransformer>,
        bearer_auth: Arc<BearerAuthenticator>,
        anonymous_registers_handler: Arc<dyn AnonymousRegistersTrait>,
        user_logins_with_password: Arc<dyn UserLoginsWithPasswordTrait>,
        user_registers_with_password: Arc<dyn UserRegistersWithPasswordTrait>,
        anonymous_requests_restore_token: Arc<dyn AnonymousRequestsRestoreTokenTrait>,
        anonymous_with_token_changes_password: Arc<dyn AnonymousWithTokenChangesPasswordTrait>,
        error_pre_handler: Arc<ErrorPreHandler>,
    ) -> Self {
        let pre_handler = Arc::new(UserAccountPreHandler::new(
            request_transformer,
            bearer_auth,
            anonymous_registers_handler,
            user_logins_with_password,
            user_registers_with_password,
            anonymous_requests_restore_token,
            anonymous_with_token_changes_password,
        ));

        let router = Arc::new(UserAccountRouter::new(
            pre_handler,
            error_pre_handler,
            "/api/v1".to_string(),
        ));

        ApiDiC { router }
    }
}
