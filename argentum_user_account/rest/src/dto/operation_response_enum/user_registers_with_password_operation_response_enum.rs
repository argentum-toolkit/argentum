use argentum_rest_infrastructure::data_type::http_response::ContentTypeResponseTrait;
use http::StatusCode;

use crate::dto::response::Status400Response;
use crate::dto::response::Status409Response;
use crate::dto::response::UserRegisteredSuccessfullyResponse;

pub enum UserRegistersWithPasswordOperationResponseEnum {
    Status201(UserRegisteredSuccessfullyResponse),
    Status400(Status400Response),
    Status409(Status409Response),
}

impl UserRegistersWithPasswordOperationResponseEnum {
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            Self::Status201(_) => StatusCode::CREATED,
            Self::Status400(_) => StatusCode::BAD_REQUEST,
            Self::Status409(_) => StatusCode::CONFLICT,
        }
    }

    pub fn to_response(&self) -> Box<dyn ContentTypeResponseTrait> {
        match self {
            Self::Status201(r) => r.to_content_type_response_trait(),
            Self::Status400(r) => r.to_content_type_response_trait(),
            Self::Status409(r) => r.to_content_type_response_trait(),
        }
    }
}
