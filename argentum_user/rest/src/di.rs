use crate::server::handler::GetUserTrait;
use crate::server::{Router, UserAccountPreHandler};
use argentum_rest_infrastructure::service::{
    BearerAuthenticator, ErrorPreHandler, RequestTransformer,
};
use std::sync::Arc;

pub struct ApiDiC {
    pub router: Arc<Router>,
}

impl ApiDiC {
    pub fn new(
        //config
        url_prefix: String,
        //services
        request_transformer: Arc<RequestTransformer>,
        bearer_auth: Arc<BearerAuthenticator>,
        get_user_handler: Arc<dyn GetUserTrait>,
        error_pre_handler: Arc<ErrorPreHandler>,
    ) -> Self {
        let pre_handler = Arc::new(UserAccountPreHandler::new(
            request_transformer,
            bearer_auth,
            get_user_handler,
        ));

        let router = Arc::new(Router::new(pre_handler, error_pre_handler, url_prefix));

        ApiDiC { router }
    }
}
