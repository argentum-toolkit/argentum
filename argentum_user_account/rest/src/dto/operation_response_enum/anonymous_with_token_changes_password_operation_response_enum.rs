use argentum_rest_infrastructure::data_type::http_response::ContentTypeResponseTrait;
use hyper::StatusCode;

use crate::dto::response::EmptyOkResponse;
use crate::dto::response::Status400Response;
use crate::dto::response::Status401Response;

pub enum AnonymousWithTokenChangesPasswordOperationResponseEnum {
    Status200(EmptyOkResponse),
    Status400(Status400Response),
    Status401(Status401Response),
}

impl AnonymousWithTokenChangesPasswordOperationResponseEnum {
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            Self::Status200(_) => {
                StatusCode::from_u16(200).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Self::Status400(_) => {
                StatusCode::from_u16(400).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Self::Status401(_) => {
                StatusCode::from_u16(401).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub fn to_response(&self) -> Box<dyn ContentTypeResponseTrait> {
        match self {
            Self::Status200(r) => r.to_content_type_response_trait(),
            Self::Status400(r) => r.to_content_type_response_trait(),
            Self::Status401(r) => r.to_content_type_response_trait(),
        }
    }
}
