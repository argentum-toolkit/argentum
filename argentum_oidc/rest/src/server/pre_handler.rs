use crate::dto::request::ExchangeCodeRequest;

use crate::server::handler::ExchangeCodeTrait;
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, RequestTrait};
use argentum_rest_infrastructure::service::RequestTransformer;
use argentum_standard_business::data_type::id::Id;
use std::collections::HashMap;
use std::sync::Arc;

pub struct PreHandler<I>
where
    I: Id,
{
    request_transformer: Arc<RequestTransformer>,
    exchange_code: Arc<dyn ExchangeCodeTrait<Id = I>>,
}

impl<I> PreHandler<I>
where
    I: Id,
{
    pub fn new(
        request_transformer: Arc<RequestTransformer>,
        exchange_code: Arc<dyn ExchangeCodeTrait<Id = I>>,
    ) -> Self {
        Self {
            request_transformer,
            exchange_code,
        }
    }

    pub async fn exchange_code(
        &self,
        request: impl RequestTrait,
        raw_path_params: HashMap<&str, &str>,
    ) -> Result<HttpResponse, HttpError> {
        let raw_query_params = HashMap::from([]);
        let req: ExchangeCodeRequest = self
            .request_transformer
            .transform(request, raw_path_params, raw_query_params)
            .await?;
        let r = self.exchange_code.handle(req).await?;

        Ok(HttpResponse::new(
            r.to_status_code(),
            r.to_response().body(),
        ))
    }
}
