use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};

use crate::dto::operation_response_enum::AnonymousRegistersOperationResponseEnum;

pub trait AnonymousRegistersTrait: Send + Sync {
    fn handle(&self) -> Result<AnonymousRegistersOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
