use crate::api::server::UserAccountPreHandler;
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, Request};
use argentum_rest_infrastructure::service::{ErrorPreHandler, Router};
use async_trait::async_trait;
use hyper::Method;
use std::sync::Arc;

pub struct UserAccountRouter {
    pub pre_handler: Arc<UserAccountPreHandler>,
    pub error_pre_handler: Arc<ErrorPreHandler>,
}

impl UserAccountRouter {
    pub fn new(
        pre_handler: Arc<UserAccountPreHandler>,
        error_pre_handler: Arc<ErrorPreHandler>,
    ) -> Self {
        UserAccountRouter {
            pre_handler,
            error_pre_handler,
        }
    }
}

#[async_trait]
impl Router for UserAccountRouter {
    async fn route(&self, req: Request) -> Result<HttpResponse, HttpError> {
        let path = req.uri().path().to_owned();
        let mut segments = Vec::new();

        for s in path.split('/') {
            match s {
                "" | "." => {}
                s => segments.push(s),
            }
        }

        match segments.as_slice() {
            ["api", "v1", "user", "anonymous-register"] => match *req.method() {
                Method::POST => self.pre_handler.anonymous_registers().await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            },
            ["api", "v1", "user", "register"] => match *req.method() {
                Method::POST => self.pre_handler.user_registers_with_password(req).await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            },
            ["api", "v1", "user", "password-login"] => match *req.method() {
                Method::POST => self.pre_handler.user_logins_with_password(req).await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            },
            ["api", "v1", "user", "restore-password", "token-request"] => match *req.method() {
                Method::POST => self.pre_handler.anonymous_requests_restore_token(req).await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            },
            ["api", "v1", "user", "restore-password", "change-password"] => match *req.method() {
                Method::POST => {
                    self.pre_handler
                        .anonymous_change_password_with_token(req)
                        .await
                }
                _ => self.error_pre_handler.method_not_allowed(req).await,
            },
            _ => self.error_pre_handler.not_found(req).await,
        }
    }
}
