use argentum_rest_infrastructure::data_type::http_response::ContentTypeResponseTrait;
use hyper::StatusCode;

use crate::dto::response::Status400Response;
use crate::dto::response::Status422Response;
use crate::dto::response::UserRegisteredSuccessfullyResponse;

pub enum UserRegistersWithPasswordOperationResponseEnum {
    Status201(UserRegisteredSuccessfullyResponse),
    Status400(Status400Response),
    Status422(Status422Response),
}

impl UserRegistersWithPasswordOperationResponseEnum {
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            Self::Status201(_) => {
                StatusCode::from_u16(201).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Self::Status400(_) => {
                StatusCode::from_u16(400).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Self::Status422(_) => {
                StatusCode::from_u16(422).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub fn to_response(&self) -> Box<dyn ContentTypeResponseTrait> {
        match self {
            Self::Status201(r) => r.to_content_type_response_trait(),
            Self::Status400(r) => r.to_content_type_response_trait(),
            Self::Status422(r) => r.to_content_type_response_trait(),
        }
    }
}
