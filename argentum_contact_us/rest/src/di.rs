use crate::server::handler::GuestSendsMessageTrait;
use crate::server::handler::MemberSendsMessageTrait;

use crate::server::{PreHandler, Router};
use argentum_rest_infrastructure::service::{
    BearerAuthenticator, ErrorPreHandler, RequestTransformer,
};
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::di_container::Container;
use std::sync::Arc;

pub fn contact_us_rest_provide<I>(container: &mut Container, url_prefix: &'static str)
where
    I: Id + 'static,
{
    container.provide_factory::<Router<I>, _>(|c| {
        let request_transformer = c.resolve::<RequestTransformer>()?;
        let bearer_auth = c.resolve::<BearerAuthenticator<I>>()?;
        let guest_sends_message_handler = c.resolve::<dyn GuestSendsMessageTrait<Id = I>>()?;
        let member_sends_message_handler = c.resolve::<dyn MemberSendsMessageTrait<Id = I>>()?;
        let error_pre_handler = c.resolve::<ErrorPreHandler>()?;

        let pre_handler = Arc::new(PreHandler::new(
            request_transformer,
            bearer_auth,
            guest_sends_message_handler,
            member_sends_message_handler,
        ));

        Ok(Arc::new(Router::new(
            pre_handler,
            error_pre_handler,
            url_prefix,
        )))
    });
}
