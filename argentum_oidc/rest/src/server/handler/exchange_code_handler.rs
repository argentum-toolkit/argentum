use crate::dto::operation_response_enum::ExchangeCodeOperationResponseEnum;
use crate::dto::request::ExchangeCodeRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_standard_business::data_type::id::Id;

use async_trait::async_trait;

#[async_trait]
pub trait ExchangeCodeTrait: Send + Sync {
    type Id: Id;

    async fn handle(
        &self,
        _req: ExchangeCodeRequest,
    ) -> Result<ExchangeCodeOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
