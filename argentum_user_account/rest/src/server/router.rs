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
            None => return self.error_pre_handler.not_found(req).await,
            Some(path) => path,
        };

        let segments: Vec<_> = path.split('/').filter(|s| !s.is_empty()).collect();

        match segments.as_slice() {
            ["user","anonymous-register",] => match *req.method() {
                    Method::POST => self.pre_handler.anonymous_registers().await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            },
            ["user","password-login",] => match *req.method() {
                    Method::POST => self.pre_handler.user_logins_with_password(req).await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            },
            ["user","register",] => match *req.method() {
                    Method::POST => self.pre_handler.user_registers_with_password(req).await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            },
            ["user","restore-password","change-password",] => match *req.method() {
                    Method::POST => self.pre_handler.anonymous_with_token_changes_password(req).await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            },
            ["user","restore-password","token-request",] => match *req.method() {
                    Method::POST => self.pre_handler.anonymous_requests_restore_token(req).await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            },
            _ => self.error_pre_handler.not_found(req).await,
        }
    }
}
