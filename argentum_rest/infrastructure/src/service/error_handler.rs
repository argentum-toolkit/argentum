use crate::data_type::error::HttpError;
use crate::data_type::{HttpResponse, ProblemDetail};
use hyper::StatusCode;

pub struct ErrorHandler {}

impl ErrorHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle(&self, err: HttpError) -> HttpResponse {
        //TODO: log error and return trace-id
        match err {
            HttpError::NotImplemented(_) => {
                let code = StatusCode::NOT_IMPLEMENTED;
                HttpResponse::new(
                    code,
                    Box::new(ProblemDetail::new(None, code.to_string(), code, None, None)),
                )
            }
            HttpError::BadRequest(e) => {
                let code = StatusCode::BAD_REQUEST;
                HttpResponse::new(
                    code,
                    Box::new(ProblemDetail::new(
                        None,
                        "Bad Request".to_string(),
                        code,
                        None,
                        Some(Box::new(e)),
                    )),
                )
            }
            HttpError::NotFound(e) => {
                let code = StatusCode::NOT_FOUND;
                HttpResponse::new(
                    code,
                    Box::new(ProblemDetail::new(None, e.msg, code, None, None)),
                )
            }
            HttpError::MethodNotAllowed(e) => {
                let code = StatusCode::METHOD_NOT_ALLOWED;
                HttpResponse::new(
                    code,
                    Box::new(ProblemDetail::new(None, e.to_string(), code, None, None)),
                )
            }
            HttpError::InternalServerError(_) => {
                let code = StatusCode::INTERNAL_SERVER_ERROR;
                HttpResponse::new(
                    code,
                    Box::new(ProblemDetail::new(None, code.to_string(), code, None, None)),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::error::{
        BadRequestError, HttpError, InternalServerError, MethodNotAllowedError, NotFoundError,
        NotImplementedError,
    };
    use crate::service::ErrorHandler;
    use argentum_standard_business::invariant_violation::Violations;
    use hyper::{Method, StatusCode};
    use serde_json::json;

    #[test]
    fn test_handle_not_implemented() {
        let handler = ErrorHandler::new();

        let response = handler.handle(HttpError::NotImplemented(NotImplementedError::new()));
        assert_eq!(response.code, StatusCode::NOT_IMPLEMENTED);

        let str = serde_json::to_value(&response.body).unwrap();

        let expected = json!({
            "type": "about:blank",
            "title": "501 Not Implemented",
            "status": 501,
            "detail": null
        });

        assert_eq!(str, expected);
    }

    #[test]
    fn test_handle_bad_request() {
        let handler = ErrorHandler::new();

        let response = handler.handle(HttpError::BadRequest(BadRequestError::new(
            Violations::new(vec![], None),
            Violations::new(vec![], None),
        )));
        assert_eq!(response.code, StatusCode::BAD_REQUEST);

        let str = serde_json::to_value(&response.body).unwrap();

        let expected = json!({
            "type": "about:blank",
            "title": "Bad Request",
            "status": 400,
            "detail": null
        });

        assert_eq!(str, expected);
    }

    #[test]
    fn test_handle_not_found() {
        let handler = ErrorHandler::new();

        let response = handler.handle(HttpError::NotFound(NotFoundError::new(
            "Entity Not Found".to_string(),
        )));
        assert_eq!(response.code, StatusCode::NOT_FOUND);

        let str = serde_json::to_value(&response.body).unwrap();

        let expected = json!({
            "type": "about:blank",
            "title": "Entity Not Found",
            "status": 404,
            "detail": null
        });

        assert_eq!(str, expected);
    }

    #[test]
    fn test_handle_method_nod_allowed() {
        let handler = ErrorHandler::new();

        let response = handler.handle(HttpError::MethodNotAllowed(MethodNotAllowedError::new(
            Method::DELETE,
        )));
        assert_eq!(response.code, StatusCode::METHOD_NOT_ALLOWED);

        let str = serde_json::to_value(&response.body).unwrap();

        let expected = json!({
            "type": "about:blank",
            "title": "Method DELETE not allowed for this endpoint",
            "status": 405,
            "detail": null
        });

        assert_eq!(str, expected);
    }

    #[test]
    fn test_handle_internal_server_error() {
        let handler = ErrorHandler::new();

        let response = handler.handle(HttpError::InternalServerError(InternalServerError::new(
            Box::new(ErrorMock {}),
        )));
        assert_eq!(response.code, StatusCode::INTERNAL_SERVER_ERROR);

        let str = serde_json::to_value(&response.body).unwrap();

        let expected = json!({
            "type": "about:blank",
            "title": "500 Internal Server Error",
            "status": 500,
            "detail": null
        });

        assert_eq!(str, expected);
    }

    #[derive(thiserror::Error, Debug)]
    #[error("test error")]
    struct ErrorMock {}
}
