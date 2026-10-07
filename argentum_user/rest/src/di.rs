use crate::server::handler::GetUserTrait;
use crate::server::{PreHandler, Router};
use argentum_rest_infrastructure::service::{
    BearerAuthenticator, ErrorPreHandler, RequestTransformer,
};
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::di_container::Container;
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
        url_prefix: &str,
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

pub fn user_api_provide<I>(container: &mut Container, url_prefix: &'static str)
where
    I: Id + 'static,
{
    container.provide_factory::<Router<_>, _>(|c| {
        let request_transformer = c.resolve::<RequestTransformer>()?;
        let bearer_auth = c.resolve::<BearerAuthenticator<I>>()?;
        let get_user_handler = c.resolve::<dyn GetUserTrait<Id = I>>()?;

        let error_pre_handler = c.resolve::<ErrorPreHandler>()?;

        let pre_handler = Arc::new(PreHandler::new(
            request_transformer,
            bearer_auth,
            get_user_handler,
        ));

        Ok(Arc::new(Router::new(
            pre_handler,
            error_pre_handler,
            url_prefix,
        )))
    });
}
