use crate::server::PreHandler;
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, Request};
use argentum_rest_infrastructure::service::{ErrorPreHandler, RouterTrait};
use async_trait::async_trait;
use hyper::{Method, Uri};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

static REGEX_USER_ACCOUNT_ANONYMOUS_REGISTER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\/user-account\/anonymous-register$").unwrap());

static REGEX_USER_ACCOUNT_PASSWORD_LOGIN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\/user-account\/password-login$").unwrap());

static REGEX_USER_ACCOUNT_REGISTER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\/user-account\/register$").unwrap());

static REGEX_USER_ACCOUNT_RESTORE_PASSWORD_TOKEN_REQUEST: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\/user-account\/restore-password\/token-request$").unwrap());

static REGEX_USER_RESTORE_PASSWORD_CHANGE_PASSWORD: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\/user\/restore-password\/change-password$").unwrap());

pub struct Router {
    pre_handler: Arc<PreHandler>,
    error_pre_handler: Arc<ErrorPreHandler>,
    url_prefix: String,
}

impl Router {
    pub fn new(
        pre_handler: Arc<PreHandler>,
        error_pre_handler: Arc<ErrorPreHandler>,
        url_prefix: String,
    ) -> Self {
        Self {
            pre_handler,
            error_pre_handler,
            url_prefix,
        }
    }
}

#[async_trait]
impl RouterTrait for Router {
    fn is_route_supported(&self, uri: &Uri, method: &Method) -> bool {
        let path = uri.path();
        let path = match path.strip_prefix(self.url_prefix.as_str()) {
            None => return false,
            Some(path) => path,
        };

        if let Some(_) = REGEX_USER_ACCOUNT_ANONYMOUS_REGISTER.captures(path) {
            return match *method {
                Method::POST => true,
                _ => false,
            };
        }

        if let Some(_) = REGEX_USER_ACCOUNT_PASSWORD_LOGIN.captures(path) {
            return match *method {
                Method::POST => true,
                _ => false,
            };
        }

        if let Some(_) = REGEX_USER_ACCOUNT_REGISTER.captures(path) {
            return match *method {
                Method::POST => true,
                _ => false,
            };
        }

        if let Some(_) = REGEX_USER_ACCOUNT_RESTORE_PASSWORD_TOKEN_REQUEST.captures(path) {
            return match *method {
                Method::POST => true,
                _ => false,
            };
        }

        if let Some(_) = REGEX_USER_RESTORE_PASSWORD_CHANGE_PASSWORD.captures(path) {
            return match *method {
                Method::POST => true,
                _ => false,
            };
        }

        false
    }

    async fn route(&self, req: Request) -> Result<HttpResponse, HttpError> {
        let path = req.uri().path();
        let path = match path.strip_prefix(self.url_prefix.as_str()) {
            None => return self.error_pre_handler.route_not_found(req).await,
            Some(path) => path,
        };

        if let Some(_) = REGEX_USER_ACCOUNT_ANONYMOUS_REGISTER.captures(path) {
            let raw_path_params = HashMap::from([]);

            return match *req.method() {
                Method::POST => {
                    self.pre_handler
                        .anonymous_registers(req, raw_path_params)
                        .await
                }
                _ => self.error_pre_handler.method_not_allowed(req).await,
            };
        }

        if let Some(_) = REGEX_USER_ACCOUNT_PASSWORD_LOGIN.captures(path) {
            let raw_path_params = HashMap::from([]);

            return match *req.method() {
                Method::POST => {
                    self.pre_handler
                        .user_logins_with_password(req, raw_path_params)
                        .await
                }
                _ => self.error_pre_handler.method_not_allowed(req).await,
            };
        }

        if let Some(_) = REGEX_USER_ACCOUNT_REGISTER.captures(path) {
            let raw_path_params = HashMap::from([]);

            return match *req.method() {
                Method::POST => {
                    self.pre_handler
                        .user_registers_with_password(req, raw_path_params)
                        .await
                }
                _ => self.error_pre_handler.method_not_allowed(req).await,
            };
        }

        if let Some(_) = REGEX_USER_ACCOUNT_RESTORE_PASSWORD_TOKEN_REQUEST.captures(path) {
            let raw_path_params = HashMap::from([]);

            return match *req.method() {
                Method::POST => {
                    self.pre_handler
                        .anonymous_requests_restore_token(req, raw_path_params)
                        .await
                }
                _ => self.error_pre_handler.method_not_allowed(req).await,
            };
        }

        if let Some(_) = REGEX_USER_RESTORE_PASSWORD_CHANGE_PASSWORD.captures(path) {
            let raw_path_params = HashMap::from([]);

            return match *req.method() {
                Method::POST => {
                    self.pre_handler
                        .anonymous_with_token_changes_password(req, raw_path_params)
                        .await
                }
                _ => self.error_pre_handler.method_not_allowed(req).await,
            };
        }

        self.error_pre_handler.route_not_found(req).await
    }
}
