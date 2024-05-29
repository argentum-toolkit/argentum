use crate::server::UserAccountPreHandler;
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, Request};
use argentum_rest_infrastructure::service::{ErrorPreHandler, Router};
use async_trait::async_trait;
use hyper::Method;
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
    async fn route(&self, req: Request) -> Result<HttpResponse, HttpError> {
        let path = req.uri().path();
        let path = match path.strip_prefix(self.url_prefix.as_str()) {
            None => return self.error_pre_handler.route_not_found(req).await,
            Some(path) => path,
        };

        let segments: Vec<_> = path.split('/').filter(|s| !s.is_empty()).collect();

        match segments.as_slice() {
            ["user", "{userId}"] => match *req.method() {
                Method::GET => self.pre_handler.get_user().await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            },
            _ => self.error_pre_handler.route_not_found(req).await,
        }
    }
}
