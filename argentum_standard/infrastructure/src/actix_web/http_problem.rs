use actix_web::http::StatusCode;
use actix_web::{error, HttpRequest, HttpResponse};

#[derive(serde::Serialize)]
pub struct HttpProblemError {
    // type: String //kind: string (url E.g.: https://example.com/problems/request-parameters-missing),
    title: String,
    // detail: String,
    code: u16,
    // invalid-params: Map<u16, Map<u16, String>>, //validation errors (alternative names `problems`)
    // trace_id: UUID
    // stack_trace
}

impl HttpProblemError {
    pub fn new(title: String, code: u16) -> HttpProblemError {
        HttpProblemError { title, code }
    }
}

///400 Bad Request
pub fn build_bad_request_response(title: String) -> HttpResponse {
    HttpResponse::BadRequest()
        .content_type("application/problem+json")
        .json(HttpProblemError::new(
            title,
            StatusCode::BAD_REQUEST.as_u16(),
        ))
}

///404 Not Found
pub fn build_not_found_response(title: String) -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("application/problem+json")
        .json(HttpProblemError::new(title, StatusCode::NOT_FOUND.as_u16()))
}

///422 Unprocessable Entity
pub fn build_unprocessable_entity_response(title: String) -> HttpResponse {
    HttpResponse::UnprocessableEntity()
        .content_type("application/problem+json")
        .json(HttpProblemError::new(
            title,
            StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
        ))
}

///500 Internal Server Error
pub fn build_internal_server_error_response() -> HttpResponse {
    HttpResponse::InternalServerError()
        .content_type("application/problem+json")
        .json(HttpProblemError::new(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        ))
}

///501 Not Implemented
pub fn build_not_implemented_response() -> HttpResponse {
    HttpResponse::NotImplemented()
        .content_type("application/problem+json")
        .json(HttpProblemError::new(
            "Not Implemented".to_string(),
            StatusCode::NOT_IMPLEMENTED.as_u16(),
        ))
}

pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let title = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType()
            .content_type("application/problem+json")
            .json(HttpProblemError::new(
                title,
                StatusCode::UNSUPPORTED_MEDIA_TYPE.as_u16(),
            )),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            build_unprocessable_entity_response(title)
        }

        _ => build_bad_request_response(title),
    };

    error::InternalError::from_response(err, resp).into()
}
