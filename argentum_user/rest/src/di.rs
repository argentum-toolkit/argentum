use crate::server::handler::GetUserTrait;
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
        get_user_handler: Arc<dyn GetUserTrait<Id = I>>,
        error_pre_handler: Arc<ErrorPreHandler>,
    ) -> Self {
        let pre_handler = Arc::new(PreHandler::new(
            request_transformer,
            bearer_auth,
            get_user_handler,
        ));

        let router = Arc::new(Router::new(pre_handler, error_pre_handler, url_prefix));

        ApiDi { router }
    }
}
