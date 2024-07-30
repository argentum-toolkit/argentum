use crate::dto::request::GetUserRequest;

use crate::server::handler::GetUserTrait;
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, RequestTrait};
use argentum_rest_infrastructure::service::{BearerAuthenticator, RequestTransformer};
use std::collections::HashMap;
use std::sync::Arc;

pub struct UserAccountPreHandler {
    request_transformer: Arc<RequestTransformer>,
    bearer_auth: Arc<BearerAuthenticator>,
    get_user: Arc<dyn GetUserTrait>,
}

impl UserAccountPreHandler {
    pub fn new(
        request_transformer: Arc<RequestTransformer>,
        bearer_auth: Arc<BearerAuthenticator>,
        get_user: Arc<dyn GetUserTrait>,
    ) -> Self {
        UserAccountPreHandler {
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
        let user = self.bearer_auth.auth(&req.params.headers)?;
        let r = self.get_user.handle(req, user)?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }
}
