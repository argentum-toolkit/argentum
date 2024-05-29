use crate::data_type::error::HttpError::{MethodNotAllowed, RouteNotFound};
use crate::data_type::error::{HttpError, MethodNotAllowedError, NotFoundError};
use crate::data_type::{HttpResponse, RequestTrait};

#[derive(Default)]
pub struct ErrorPreHandler {}

impl ErrorPreHandler {
    pub fn new() -> ErrorPreHandler {
        ErrorPreHandler {}
    }

    pub async fn route_not_found(
        &self,
        _request: impl RequestTrait,
    ) -> Result<HttpResponse, HttpError> {
        Err(RouteNotFound(NotFoundError::new(
            "Resource not found".to_string(),
        )))
    }

    pub async fn method_not_allowed(
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
            .not_found(EmptyRequestMock {
                method: Method::GET,
            })
            .await;

        assert!(res.is_err());

        assert!(matches!(res, Err(HttpError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_handle_not_allowed() {
        let handler = ErrorPreHandler::new();
        let res = handler
            .method_not_allowed(EmptyRequestMock {
                method: Method::GET,
            })
            .await;

        assert!(res.is_err());

        assert!(matches!(res, Err(HttpError::MethodNotAllowed(_))));

        if let Err(HttpError::MethodNotAllowed(e)) = res {
            assert_eq!(e.method, Method::GET);
        }

        let res = handler
            .method_not_allowed(EmptyRequestMock {
                method: Method::OPTIONS,
            })
            .await;

        assert!(res.is_err());

        assert!(matches!(res, Err(HttpError::MethodNotAllowed(_))));

        if let Err(HttpError::MethodNotAllowed(e)) = res {
            assert_eq!(e.method, Method::OPTIONS);
        }
    }
}
