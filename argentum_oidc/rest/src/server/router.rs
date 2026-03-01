use crate::server::PreHandler;
use argentum_rest_infrastructure::data_type::error::HttpError;
use argentum_rest_infrastructure::data_type::{HttpResponse, Request};
use argentum_rest_infrastructure::service::{ErrorPreHandler, RouterTrait};
use argentum_standard_business::data_type::id::Id;
use async_trait::async_trait;
use compiletime_regex::regex;
use hyper::{Method, Uri};
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Router<I>
where
    I: Id,
{
    pre_handler: Arc<PreHandler<I>>,
    error_pre_handler: Arc<ErrorPreHandler>,
    url_prefix: String,
    regex_exchange_code: Regex,
}

impl<I> Router<I>
where
    I: Id,
{
    pub fn new(
        pre_handler: Arc<PreHandler<I>>,
        error_pre_handler: Arc<ErrorPreHandler>,
        url_prefix: &str,
    ) -> Self {
        let regex_exchange_code = regex!(r"\/exchange-code$");

        Self {
            pre_handler,
            error_pre_handler,
            url_prefix: url_prefix.into(),
            regex_exchange_code,
        }
    }
}

#[async_trait]
impl<I> RouterTrait for Router<I>
where
    I: Id,
{
    fn is_route_supported(&self, uri: &Uri, method: &Method) -> bool {
        let path = uri.path();
        let path = match path.strip_prefix(self.url_prefix.as_str()) {
            None => return false,
            Some(path) => path,
        };

        if self.regex_exchange_code.captures(path).is_some() {
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

        if self.regex_exchange_code.captures(path).is_some() {
            let raw_path_params = HashMap::new();

            return match *req.method() {
                Method::POST => self.pre_handler.exchange_code(req, raw_path_params).await,
                _ => self.error_pre_handler.method_not_allowed(req).await,
            };
        }

        self.error_pre_handler.route_not_found(req).await
    }
}
