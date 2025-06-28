use argentum_rest_infrastructure::data_type::http_response::ContentTypeResponseTrait;
use http::StatusCode;

use crate::dto::response::Status400Response;
use crate::dto::response::Status401Response;
use crate::dto::response::UserLoggedInSuccessfullyResponse;

pub enum UserLoginsWithPasswordOperationResponseEnum {
    Status200(UserLoggedInSuccessfullyResponse),
    Status400(Status400Response),
    Status401(Status401Response),
}

impl UserLoginsWithPasswordOperationResponseEnum {
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            Self::Status200(_) => StatusCode::OK,
            Self::Status400(_) => StatusCode::BAD_REQUEST,
            Self::Status401(_) => StatusCode::UNAUTHORIZED,
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
