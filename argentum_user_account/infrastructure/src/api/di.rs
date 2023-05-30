use crate::api::server::handler::{AnonymousRegistersTrait, UserRegistersWithPasswordTrait};
use crate::api::server::{ToDoRouter, TodoPreHandler};
use argentum_rest_infrastructure::service::{ErrorPreHandler, RequestTransformer};
use std::sync::Arc;

pub struct ApiDiC {
    pub router: Arc<ToDoRouter>,
}

impl ApiDiC {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        request_transformer: Arc<RequestTransformer>,
        anonymous_registers_handler: Arc<dyn AnonymousRegistersTrait>,
        user_registers_with_password: Arc<dyn UserRegistersWithPasswordTrait>,
        error_pre_handler: Arc<ErrorPreHandler>,
    ) -> Self {
        let pre_handler = Arc::new(TodoPreHandler::new(
            request_transformer,
            anonymous_registers_handler,
            user_registers_with_password,
        ));

        let router = Arc::new(ToDoRouter::new(pre_handler, error_pre_handler));

        ApiDiC { router }
    }
}
