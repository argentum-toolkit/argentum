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
    regex_user_userid: Regex,
}

impl Router {
    pub fn new(
        pre_handler: Arc<PreHandler>,
        error_pre_handler: Arc<ErrorPreHandler>,
        url_prefix: String,
    ) -> Self {
        let regex_user_userid = regex!(
            r"\/user\/(?<userId>[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})$"
        );

        Self {
            pre_handler,
            error_pre_handler,
            url_prefix,
            regex_user_userid,
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

        if self.regex_user_userid.captures(path).is_some() {
            return match *method {
                Method::GET => true,
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

        if let Some(caps) = self.regex_user_userid.captures(path) {
            let user_id = caps["userId"].to_string();

            let raw_path_params = HashMap::from([("user_id", user_id.as_str())]);

            return match *req.method() {
                Method::GET => self.pre_handler.get_user(req, raw_path_params).await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            };
        }

        self.error_pre_handler.route_not_found(req).await
    }
}
