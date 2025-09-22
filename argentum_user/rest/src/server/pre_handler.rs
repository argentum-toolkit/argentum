use crate::dto::request::GetUserRequest;

use crate::server::handler::GetUserTrait;
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
    get_user: Arc<dyn GetUserTrait<Id = I>>,
}

impl<I> PreHandler<I>
where
    I: Id,
{
    pub fn new(
        request_transformer: Arc<RequestTransformer>,
        bearer_auth: Arc<BearerAuthenticator<I>>,
        get_user: Arc<dyn GetUserTrait<Id = I>>,
    ) -> Self {
        Self {
            request_transformer,
            bearer_auth,
            get_user,
        }
    }

    pub async fn get_user(
        &self,
        request: impl RequestTrait,
        raw_path_params: HashMap<&str, &str>,
    ) -> Result<HttpResponse, HttpError> {
        let raw_query_params = HashMap::from([]);
        let req: GetUserRequest = self
            .request_transformer
            .transform(request, raw_path_params, raw_query_params)
            .await?;
        let user = self.bearer_auth.auth(&req.params.headers).await?;
        let r = self.get_user.handle(req, user).await?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }
}
