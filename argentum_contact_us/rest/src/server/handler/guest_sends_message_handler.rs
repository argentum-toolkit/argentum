use crate::dto::operation_response_enum::GuestSendsMessageOperationResponseEnum;
use crate::dto::request::GuestSendsMessageRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_standard_business::data_type::id::Id;

use async_trait::async_trait;

#[async_trait]
pub trait GuestSendsMessageTrait: Send + Sync {
    type Id: Id;

    async fn handle(
        &self,
        _req: GuestSendsMessageRequest,
    ) -> Result<GuestSendsMessageOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
