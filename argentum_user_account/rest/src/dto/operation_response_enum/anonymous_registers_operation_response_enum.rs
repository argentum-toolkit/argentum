use argentum_rest_infrastructure::data_type::{
    http_response::ContentTypeResponseTrait, http_status, http_status::StatusCode,
};

use crate::dto::response::AnonymousRegisteredSuccessfullyResponse;

pub enum AnonymousRegistersOperationResponseEnum {
    Status201(AnonymousRegisteredSuccessfullyResponse),
}

impl AnonymousRegistersOperationResponseEnum {
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            Self::Status201(_) => http_status::CREATED,
        }
    }

    pub fn to_response(&self) -> Box<dyn ContentTypeResponseTrait> {
        match self {
            Self::Status201(r) => r.to_content_type_response_trait(),
        }
    }
}
