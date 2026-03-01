use crate::dto::operation_response_enum::GuestRegistersOperationResponseEnum;
use crate::dto::request::GuestRegistersRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_standard_business::data_type::id::Id;

use async_trait::async_trait;

#[async_trait]
pub trait GuestRegistersTrait: Send + Sync {
    type Id: Id;

    async fn handle(
        &self,
        _req: GuestRegistersRequest,
    ) -> Result<GuestRegistersOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
