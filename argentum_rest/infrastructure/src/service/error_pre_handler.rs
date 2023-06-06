use crate::data_type::error::HttpError::{MethodNotAllowed, NotFound};
use crate::data_type::error::{HttpError, MethodNotAllowedError, NotFoundError};
use crate::data_type::{HttpResponse, RequestTrait};

#[derive(Default)]
pub struct ErrorPreHandler {}

impl ErrorPreHandler {
    pub fn new() -> ErrorPreHandler {
        ErrorPreHandler {}
    }

    pub async fn handle_not_found(
        &self,
        _request: impl RequestTrait,
    ) -> Result<HttpResponse, HttpError> {
        Err(NotFound(NotFoundError::new(
            "Resource not found".to_string(),
        )))
    }

    pub async fn handle_method_not_allowed(
        &self,
        request: impl RequestTrait,
    ) -> Result<HttpResponse, HttpError> {
        Err(MethodNotAllowed(MethodNotAllowedError::new(
            request.method().clone(),
        )))
    }
}
#[cfg(test)]
mod tests {
    use crate::data_type::error::HttpError;
    use crate::data_type::RequestTrait;
    use crate::service::ErrorPreHandler;
    use async_trait::async_trait;
    use hyper::{Error, HeaderMap, Method};

    struct EmptyRequestMock {
        method: Method,
    }
    #[async_trait]
    impl RequestTrait for EmptyRequestMock {
        async fn fetch_body(self) -> Result<Vec<u8>, Error> {
            Ok(vec![])
        }

        fn method(&self) -> &Method {
            &self.method
        }

        fn get_headers(&self) -> &HeaderMap {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_handle_not_found() {
        let handler = ErrorPreHandler::new();
        let res = handler
            .handle_not_found(EmptyRequestMock {
                method: Method::GET,
            })
            .await;

        assert_eq!(res.is_err(), true);

        assert!(if let Err(HttpError::NotFound(_)) = res {
            true
        } else {
            false
        });
    }

    #[tokio::test]
    async fn test_handle_not_allowed() {
        let handler = ErrorPreHandler::new();
        let res = handler
            .handle_method_not_allowed(EmptyRequestMock {
                method: Method::GET,
            })
            .await;

        assert_eq!(res.is_err(), true);

        assert!(if let Err(HttpError::MethodNotAllowed(_)) = res {
            true
        } else {
            false
        });

        if let Err(HttpError::MethodNotAllowed(e)) = res {
            assert_eq!(e.method, Method::GET);
        }

        let res = handler
            .handle_method_not_allowed(EmptyRequestMock {
                method: Method::OPTIONS,
            })
            .await;

        assert_eq!(res.is_err(), true);

        assert!(if let Err(HttpError::MethodNotAllowed(_)) = res {
            true
        } else {
            false
        });

        if let Err(HttpError::MethodNotAllowed(e)) = res {
            assert_eq!(e.method, Method::OPTIONS);
        }
    }
}
