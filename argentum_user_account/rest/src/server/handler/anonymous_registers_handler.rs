use crate::dto::request::AnonymousRegistersRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};

use crate::dto::operation_response_enum::AnonymousRegistersOperationResponseEnum;

pub trait AnonymousRegistersTrait: Send + Sync {
    fn handle(
        &self,
        _req: AnonymousRegistersRequest,
    ) -> Result<AnonymousRegistersOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
