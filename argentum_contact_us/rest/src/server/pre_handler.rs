use crate::dto::request::GuestSendsMessageRequest;
use crate::dto::request::MemberSendsMessageRequest;

use crate::server::handler::GuestSendsMessageTrait;
use crate::server::handler::MemberSendsMessageTrait;
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, RequestTrait};
use argentum_rest_infrastructure::service::{BearerAuthenticator, RequestTransformer};
use argentum_standard_business::data_type::id::Id;
use std::collections::HashMap;
use std::sync::Arc;

pub struct PreHandler<I>
where
    I: Id,
{
    request_transformer: Arc<RequestTransformer>,
    bearer_auth: Arc<BearerAuthenticator<I>>,
    guest_sends_message: Arc<dyn GuestSendsMessageTrait<Id = I>>,
    member_sends_message: Arc<dyn MemberSendsMessageTrait<Id = I>>,
}

impl<I> PreHandler<I>
where
    I: Id,
{
    pub fn new(
        request_transformer: Arc<RequestTransformer>,
        bearer_auth: Arc<BearerAuthenticator<I>>,
        guest_sends_message: Arc<dyn GuestSendsMessageTrait<Id = I>>,
        member_sends_message: Arc<dyn MemberSendsMessageTrait<Id = I>>,
    ) -> Self {
        Self {
            request_transformer,
            bearer_auth,
            guest_sends_message,
            member_sends_message,
        }
    }

    pub async fn guest_sends_message(
        &self,
        request: impl RequestTrait,
        raw_path_params: HashMap<&str, &str>,
    ) -> Result<HttpResponse, HttpError> {
        let raw_query_params = HashMap::from([]);
        let req: GuestSendsMessageRequest = self
            .request_transformer
            .transform(request, raw_path_params, raw_query_params)
            .await?;
        let r = self.guest_sends_message.handle(req).await?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }

    pub async fn member_sends_message(
        &self,
        request: impl RequestTrait,
        raw_path_params: HashMap<&str, &str>,
    ) -> Result<HttpResponse, HttpError> {
        let raw_query_params = HashMap::from([]);
        let req: MemberSendsMessageRequest = self
            .request_transformer
            .transform(request, raw_path_params, raw_query_params)
            .await?;
        let user = self.bearer_auth.auth(&req.params.headers).await?;
        let r = self.member_sends_message.handle(req, user).await?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }
}
