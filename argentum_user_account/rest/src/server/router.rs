use crate::server::PreHandler;
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, Request};
use argentum_rest_infrastructure::service::{ErrorPreHandler, RouterTrait};
use async_trait::async_trait;
use compiletime_regex::regex;
use hyper::{Method, Uri};
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Router {
    pre_handler: Arc<PreHandler>,
    error_pre_handler: Arc<ErrorPreHandler>,
    url_prefix: String,
    regex_user_account_anonymous_register: Regex,
    regex_user_account_password_login: Regex,
    regex_user_account_register: Regex,
    regex_user_account_restore_password_token_request: Regex,
    regex_user_restore_password_change_password: Regex,
}

impl Router {
    pub fn new(
        pre_handler: Arc<PreHandler>,
        error_pre_handler: Arc<ErrorPreHandler>,
        url_prefix: String,
    ) -> Self {
        let regex_user_account_anonymous_register = regex!(r"\/user-account\/anonymous-register$");
        let regex_user_account_password_login = regex!(r"\/user-account\/password-login$");
        let regex_user_account_register = regex!(r"\/user-account\/register$");
        let regex_user_account_restore_password_token_request =
            regex!(r"\/user-account\/restore-password\/token-request$");
        let regex_user_restore_password_change_password =
            regex!(r"\/user\/restore-password\/change-password$");

        Self {
            pre_handler,
            error_pre_handler,
            url_prefix,
            regex_user_account_anonymous_register,
            regex_user_account_password_login,
            regex_user_account_register,
            regex_user_account_restore_password_token_request,
            regex_user_restore_password_change_password,
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

        if let Some(_) = self.regex_user_account_anonymous_register.captures(path) {
            return match *method {
                Method::POST => true,
                _ => false,
            };
        }

        if let Some(_) = self.regex_user_account_password_login.captures(path) {
            return match *method {
                Method::POST => true,
                _ => false,
            };
        }

        if let Some(_) = self.regex_user_account_register.captures(path) {
            return match *method {
                Method::POST => true,
                _ => false,
            };
        }

        if let Some(_) = self
            .regex_user_account_restore_password_token_request
            .captures(path)
        {
            return match *method {
                Method::POST => true,
                _ => false,
            };
        }

        if let Some(_) = self
            .regex_user_restore_password_change_password
            .captures(path)
        {
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

        if let Some(_) = self.regex_user_account_anonymous_register.captures(path) {
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

        if let Some(_) = self.regex_user_account_password_login.captures(path) {
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

        if let Some(_) = self.regex_user_account_register.captures(path) {
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

        if let Some(_) = self
            .regex_user_account_restore_password_token_request
            .captures(path)
        {
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

        if let Some(_) = self
            .regex_user_restore_password_change_password
            .captures(path)
        {
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
