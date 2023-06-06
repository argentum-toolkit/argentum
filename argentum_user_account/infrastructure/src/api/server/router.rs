use crate::api::server::UserAccountPreHandler;
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, Request};
use argentum_rest_infrastructure::service::{ErrorPreHandler, Router};
use async_trait::async_trait;
use hyper::Method;
use std::sync::Arc;

pub struct ToDoRouter {
    pub todo_pre_handler: Arc<UserAccountPreHandler>,
    pub error_pre_handler: Arc<ErrorPreHandler>,
}

impl ToDoRouter {
    pub fn new(
        todo_pre_handler: Arc<UserAccountPreHandler>,
        error_pre_handler: Arc<ErrorPreHandler>,
    ) -> Self {
        ToDoRouter {
            todo_pre_handler,
            error_pre_handler,
        }
    }
}

#[async_trait]
impl Router for ToDoRouter {
    async fn route(&self, request: Request) -> Result<HttpResponse, HttpError> {
        let path = request.uri().path().to_owned();
        let mut segments = Vec::new();

        for s in path.split('/') {
            match s {
                "" | "." => {}
                s => segments.push(s),
            }
        }

        match segments.as_slice() {
            ["api", "v1", "user", "anonymous-register"] => match *request.method() {
                Method::POST => self.todo_pre_handler.handle_anonymous_registers().await,
                _ => {
                    self.error_pre_handler
                        .handle_method_not_allowed(request)
                        .await
                }
            },
            ["api", "v1", "user", "register"] => match *request.method() {
                Method::POST => {
                    self.todo_pre_handler
                        .handle_user_registers_with_password(request)
                        .await
                }
                _ => {
                    self.error_pre_handler
                        .handle_method_not_allowed(request)
                        .await
                }
            },
            ["api", "v1", "user", "password-login"] => match *request.method() {
                Method::POST => {
                    self.todo_pre_handler
                        .handle_user_logins_with_password(request)
                        .await
                }
                _ => {
                    self.error_pre_handler
                        .handle_method_not_allowed(request)
                        .await
                }
            },
            _ => self.error_pre_handler.handle_not_found(request).await,
        }
    }
}
