use argentum_rest_infrastructure::data_type::http_response::ContentTypeResponseTrait;
use http::StatusCode;

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
            Self::Status200(_) => StatusCode::OK,
            Self::Status401(_) => StatusCode::UNAUTHORIZED,
            Self::Status403(_) => StatusCode::FORBIDDEN,
            Self::Status404(_) => StatusCode::NOT_FOUND,
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
