use argentum_rest_infrastructure::data_type::http_response::ContentTypeResponseTrait;
use hyper::StatusCode;

use crate::dto::response::GetUserOkResponse;
use crate::dto::response::Status401Response;
use crate::dto::response::Status403Response;
use crate::dto::response::Status404Response;

pub enum GetUserOperationResponseEnum {
    Status200(GetUserOkResponse),
    Status401(Status401Response),
    Status403(Status403Response),
    Status404(Status404Response),
}

impl GetUserOperationResponseEnum {
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            Self::Status200(_) => {
                StatusCode::from_u16(200).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Self::Status401(_) => {
                StatusCode::from_u16(401).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Self::Status403(_) => {
                StatusCode::from_u16(403).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Self::Status404(_) => {
                StatusCode::from_u16(404).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub fn to_response(&self) -> Box<dyn ContentTypeResponseTrait> {
        match self {
            Self::Status200(r) => r.to_content_type_response_trait(),
            Self::Status401(r) => r.to_content_type_response_trait(),
            Self::Status403(r) => r.to_content_type_response_trait(),
            Self::Status404(r) => r.to_content_type_response_trait(),
        }
    }
}
