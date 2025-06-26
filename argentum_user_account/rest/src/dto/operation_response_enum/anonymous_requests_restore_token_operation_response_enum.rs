use argentum_rest_infrastructure::data_type::{
    http_response::ContentTypeResponseTrait, http_status, http_status::StatusCode,
};

use crate::dto::response::EmptyOkResponse;
use crate::dto::response::Status400Response;
use crate::dto::response::Status401Response;

pub enum AnonymousRequestsRestoreTokenOperationResponseEnum {
    Status200(EmptyOkResponse),
    Status400(Status400Response),
    Status401(Status401Response),
}

impl AnonymousRequestsRestoreTokenOperationResponseEnum {
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            Self::Status200(_) => http_status::OK,
            Self::Status400(_) => http_status::BAD_REQUEST,
            Self::Status401(_) => http_status::UNAUTHORIZED,
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
