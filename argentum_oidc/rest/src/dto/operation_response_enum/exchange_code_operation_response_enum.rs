use argentum_rest_infrastructure::data_type::http_response::ContentTypeResponseTrait;
use http::StatusCode;

use crate::dto::response::CodeExchangedSuccessfullyResponse;
use crate::dto::response::Status400Response;

pub enum ExchangeCodeOperationResponseEnum {
    Status201(CodeExchangedSuccessfullyResponse),
    Status400(Status400Response),
}

impl ExchangeCodeOperationResponseEnum {
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            Self::Status201(_) => {
                StatusCode::from_u16(201).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Self::Status400(_) => {
                StatusCode::from_u16(400).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub fn to_response(&self) -> Box<dyn ContentTypeResponseTrait> {
        match self {
            Self::Status201(r) => r.to_content_type_response_trait(),
            Self::Status400(r) => r.to_content_type_response_trait(),
        }
    }
}
