use crate::server::UserAccountPreHandler;
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, Request};
use argentum_rest_infrastructure::service::{ErrorPreHandler, Router};
use async_trait::async_trait;
use hyper::{Method, Uri};
use std::sync::Arc;

pub struct UserAccountRouter {
    pre_handler: Arc<UserAccountPreHandler>,
    error_pre_handler: Arc<ErrorPreHandler>,
    url_prefix: String,
}

impl UserAccountRouter {
    pub fn new(
        pre_handler: Arc<UserAccountPreHandler>,
        error_pre_handler: Arc<ErrorPreHandler>,
        url_prefix: String,
    ) -> Self {
        UserAccountRouter {
            pre_handler,
            error_pre_handler,
            url_prefix,
        }
    }
}

#[async_trait]
impl Router for UserAccountRouter {
    fn is_route_supported(&self, uri: &Uri, method: &Method) -> bool {
        let path = uri.path();
        let path = match path.strip_prefix(self.url_prefix.as_str()) {
            None => return false,
            Some(path) => path,
        };

        let segments: Vec<_> = path.split('/').filter(|s| !s.is_empty()).collect();
        // toto:  check that {userId}} is an UUID
        match segments.as_slice() {
            ["user", "{userId}"] => match *method {
                Method::GET => true,
                _ => false,
            },
            _ => false,
        }
    }

    async fn route(&self, req: Request) -> Result<HttpResponse, HttpError> {
        let path = req.uri().path();
        let path = match path.strip_prefix(self.url_prefix.as_str()) {
            None => return self.error_pre_handler.route_not_found(req).await,
            Some(path) => path,
        };

        let segments: Vec<_> = path.split('/').filter(|s| !s.is_empty()).collect();

        match segments.as_slice() {
            ["user", "{userId}"] => match *req.method() {
                Method::GET => self.pre_handler.get_user(req).await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            },
            _ => self.error_pre_handler.route_not_found(req).await,
        }
    }
}
