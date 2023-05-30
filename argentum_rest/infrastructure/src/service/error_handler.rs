use crate::data_type::error::HttpError;
use crate::data_type::{HttpResponse, ProblemDetail};
use argentum_log_business::LoggerTrait;
use hyper::StatusCode;
use std::sync::Arc;

pub struct ErrorHandler {
    logger: Arc<dyn LoggerTrait>,
}

impl ErrorHandler {
    pub fn new(logger: Arc<dyn LoggerTrait>) -> Self {
        Self { logger }
    }

    pub fn handle(&self, err: HttpError) -> HttpResponse {
        //TODO: log error and return trace-id
        match err {
            HttpError::NotImplemented(e) => {
                self.logger.info(format!("{:?}", e));

                let code = StatusCode::NOT_IMPLEMENTED;
                HttpResponse::new(
                    code,
                    Box::new(ProblemDetail::new(None, code.to_string(), code, None, None)),
                )
            }
            HttpError::BadRequest(e) => {
                self.logger.info(format!("{:?}", e));

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
                self.logger.warning(format!("{:?}", e));

                let code = StatusCode::NOT_FOUND;
                HttpResponse::new(
                    code,
                    Box::new(ProblemDetail::new(None, e.msg, code, None, None)),
                )
            }
            HttpError::MethodNotAllowed(e) => {
                self.logger.warning(format!("{:?}", e));

                let code = StatusCode::METHOD_NOT_ALLOWED;
                HttpResponse::new(
                    code,
                    Box::new(ProblemDetail::new(None, e.to_string(), code, None, None)),
                )
            }
            HttpError::Conflict(e) => {
                self.logger.info(format!("{:?}", e));

                let code = StatusCode::CONFLICT;
                HttpResponse::new(
                    code,
                    Box::new(ProblemDetail::new(
                        None,
                        e.source.to_string(),
                        code,
                        None,
                        None,
                    )),
                )
            }
            HttpError::UnprocessableEntity(e) => {
                self.logger.info(format!("{:?}", e));

                let code = StatusCode::UNPROCESSABLE_ENTITY;
                HttpResponse::new(
                    code,
                    Box::new(ProblemDetail::new(None, e.to_string(), code, None, None)),
                )
            }
            HttpError::InternalServerError(e) => {
                self.logger.error(format!("Internal server error {:?}", e));

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
