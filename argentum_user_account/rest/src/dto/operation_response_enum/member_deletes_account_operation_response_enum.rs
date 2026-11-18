use argentum_rest_infrastructure::data_type::http_response::ContentTypeResponseTrait;
use http::StatusCode;

use crate::dto::response::Status204Response;
use crate::dto::response::Status400Response;
use crate::dto::response::Status401Response;

pub enum MemberDeletesAccountOperationResponseEnum {
    Status204(Status204Response),
    Status400(Status400Response),
    Status401(Status401Response),
}

impl MemberDeletesAccountOperationResponseEnum {
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            Self::Status204(_) => {
                StatusCode::from_u16(204).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
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
            Self::Status204(r) => r.to_content_type_response_trait(),
            Self::Status400(r) => r.to_content_type_response_trait(),
            Self::Status401(r) => r.to_content_type_response_trait(),
        }
    }
}
