use crate::api::server::handler::{
    AnonymousRegistersTrait, UserLoginsWithPasswordTrait, UserRegistersWithPasswordTrait,
};
use crate::api::server::{ToDoRouter, UserAccountPreHandler};
use argentum_rest_infrastructure::service::{
    BearerAuthenticator, ErrorPreHandler, RequestTransformer,
};
use std::sync::Arc;

pub struct ApiDiC {
    pub router: Arc<ToDoRouter>,
}

impl ApiDiC {
    pub fn new(
        request_transformer: Arc<RequestTransformer>,
        bearer_auth: Arc<BearerAuthenticator>,
        anonymous_registers_handler: Arc<dyn AnonymousRegistersTrait>,
        user_logins_with_password: Arc<dyn UserLoginsWithPasswordTrait>,
        user_registers_with_password: Arc<dyn UserRegistersWithPasswordTrait>,
        error_pre_handler: Arc<ErrorPreHandler>,
    ) -> Self {
        let pre_handler = Arc::new(UserAccountPreHandler::new(
            request_transformer,
            bearer_auth,
            anonymous_registers_handler,
            user_logins_with_password,
            user_registers_with_password,
        ));

        let router = Arc::new(ToDoRouter::new(pre_handler, error_pre_handler));

        ApiDiC { router }
    }
}
