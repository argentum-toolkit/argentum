use crate::dto::request::AnonymousRegistersRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_standard_business::data_type::id::Id;

use crate::dto::operation_response_enum::AnonymousRegistersOperationResponseEnum;

use async_trait::async_trait;

#[async_trait]
pub trait AnonymousRegistersTrait: Send + Sync {
    type Id: Id;

    async fn handle(
        &self,
        _req: AnonymousRegistersRequest,
    ) -> Result<AnonymousRegistersOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
