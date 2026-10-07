use crate::server::handler::ExchangeCodeTrait;
use crate::server::{PreHandler, Router};
use argentum_rest_infrastructure::service::{
    BearerAuthenticator, ErrorPreHandler, RequestTransformer,
};
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::di_container::Container;
use std::sync::Arc;

pub fn oidc_api_provide<I>(container: &mut Container, url_prefix: &'static str)
where
    I: Id + 'static,
{
    container.provide_factory::<Router<I>, _>(|c| {
        let request_transformer = c.resolve::<RequestTransformer>()?;
        let bearer_auth = c.resolve::<BearerAuthenticator<I>>()?;
        let exchange_code_handler = c.resolve::<dyn ExchangeCodeTrait<Id = I>>()?;
        let error_pre_handler = c.resolve::<ErrorPreHandler>()?;

        let pre_handler = Arc::new(PreHandler::new(
            request_transformer,
            bearer_auth,
            exchange_code_handler,
        ));

        Ok(Arc::new(Router::new(
            pre_handler,
            error_pre_handler,
            url_prefix,
        )))
    });
}
