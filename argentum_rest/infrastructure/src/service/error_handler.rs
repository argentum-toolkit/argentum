use crate::data_type::error::HttpError;
use crate::data_type::http_status;
use crate::data_type::{HttpResponse, ProblemDetail};
use argentum_log_business::LoggerTrait;
use std::sync::Arc;

pub struct ErrorHandler {
    logger: Arc<dyn LoggerTrait>,
}

impl ErrorHandler {
    pub fn new(logger: Arc<dyn LoggerTrait>) -> Self {
        Self { logger }
    }

    pub fn handle(&self, err: HttpError) -> HttpResponse {
        match err {
            HttpError::NotImplemented(e) => {
                self.logger.info(format!("{:?}", e));

                let code = http_status::NOT_IMPLEMENTED;
                HttpResponse::new(
                    code.clone(),
                    Box::new(ProblemDetail::new(
                        None,
                        code.reason.to_string(),
                        code,
                        None,
                        None,
                    )),
                )
            }
            HttpError::BadRequest(e) => {
                self.logger.info(format!("{:?}", e));

                let code = http_status::BAD_REQUEST;
                HttpResponse::new(
                    code.clone(),
                    Box::new(ProblemDetail::new(
                        None,
                        "Bad Request".to_string(),
                        code,
                        None,
                        Some(Box::new(e)),
                    )),
                )
            }
            HttpError::Unauthorized(e) => {
                self.logger.info(format!("{:?}", e));

                let code = http_status::UNAUTHORIZED;
                HttpResponse::new(
                    code.clone(),
                    Box::new(ProblemDetail::new(
                        None,
                        "Unauthorized".to_string(),
                        code,
                        Some(e.msg),
                        None,
                    )),
                )
            }
            HttpError::NotFound(e) | HttpError::RouteNotFound(e) => {
                self.logger.warning(format!("{:?}", e));

                let code = http_status::NOT_FOUND;
                HttpResponse::new(
                    code.clone(),
                    Box::new(ProblemDetail::new(None, e.msg, code, None, None)),
                )
            }
            HttpError::MethodNotAllowed(e) => {
                self.logger.warning(format!("{:?}", e));

                let code = http_status::METHOD_NOT_ALLOWED;
                HttpResponse::new(
                    code.clone(),
                    Box::new(ProblemDetail::new(None, e.to_string(), code, None, None)),
                )
            }
            HttpError::Conflict(e) => {
                self.logger.info(format!("{:?}", e));

                let code = http_status::CONFLICT;
                HttpResponse::new(
                    code.clone(),
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

                let code = http_status::UNPROCESSABLE_CONTENT;
                HttpResponse::new(
                    code.clone(),
                    Box::new(ProblemDetail::new(None, e.to_string(), code, None, None)),
                )
            }
            HttpError::InternalServerError(e) => {
                self.logger.error(format!("Internal server error {:?}", e));

                let code = http_status::INTERNAL_SERVER_ERROR;
                HttpResponse::new(
                    code.clone(),
                    Box::new(ProblemDetail::new(
                        None,
                        code.reason.to_string(),
                        code,
                        None,
                        None,
                    )),
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
    use crate::data_type::http_status;
    use crate::service::ErrorHandler;
    use argentum_log_business::{DefaultLogger, Level, StdoutWriter};
    use argentum_standard_business::invariant_violation::Violations;
    use hyper::Method;
    use serde_json::json;
    use std::sync::Arc;

    #[test]
    fn test_handle_not_implemented() {
        let log_writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));
        let handler = ErrorHandler::new(logger);

        let response = handler.handle(HttpError::NotImplemented(NotImplementedError::new()));
        assert_eq!(response.code.code, http_status::NOT_IMPLEMENTED.code);

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
        let log_writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));
        let handler = ErrorHandler::new(logger);

        let response = handler.handle(HttpError::BadRequest(BadRequestError::new(
            Violations::new(vec![], None),
            Violations::new(vec![], None),
            Violations::new(vec![], None),
            Violations::new(vec![], None),
        )));
        assert_eq!(response.code.code, http_status::BAD_REQUEST.code);

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
        let log_writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));
        let handler = ErrorHandler::new(logger);

        let response = handler.handle(HttpError::NotFound(NotFoundError::new(
            "Entity Not Found".to_string(),
        )));
        assert_eq!(response.code.code, http_status::NOT_FOUND.code);

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
        let log_writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));
        let handler = ErrorHandler::new(logger);

        let response = handler.handle(HttpError::MethodNotAllowed(MethodNotAllowedError::new(
            Method::DELETE,
        )));
        assert_eq!(response.code.code, http_status::METHOD_NOT_ALLOWED.code); //TODO fix .code. replace it to something more clear

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
        let log_writer = Arc::new(StdoutWriter::new());
        let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));
        let handler = ErrorHandler::new(logger);

        let response = handler.handle(HttpError::InternalServerError(InternalServerError::new(
            Box::new(ErrorMock {}),
        )));
        assert_eq!(response.code.code, http_status::INTERNAL_SERVER_ERROR.code);

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
