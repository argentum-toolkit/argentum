use crate::data_type::error::HttpError;
use crate::data_type::{HttpResponse, Request};
use crate::service::{ErrorPreHandler, Router};
use async_trait::async_trait;
use hyper::{Method, Uri};
use std::sync::Arc;

pub struct RouterCombinator {
    routers: Vec<Arc<dyn Router>>,
    error_pre_handler: Arc<ErrorPreHandler>,
}

impl RouterCombinator {
    pub fn new(routers: Vec<Arc<dyn Router>>, error_pre_handler: Arc<ErrorPreHandler>) -> Self {
        RouterCombinator {
            routers,
            error_pre_handler,
        }
    }
}

#[async_trait]
impl Router for RouterCombinator {
    fn is_route_supported(&self, uri: &Uri, method: &Method) -> bool {
        for r in &self.routers {
            if r.is_route_supported(uri, method) {
                return true;
            }
        }

        false
    }

    async fn route(&self, request: Request) -> Result<HttpResponse, HttpError> {
        let uri = request.uri();
        let method = request.method();
        let mut router: Option<&Arc<dyn Router>> = None;

        for r in &self.routers {
            if r.is_route_supported(uri, method) {
                router = Some(r);
            }
        }

        match router {
            None => self.error_pre_handler.route_not_found(request).await,
            Some(router) => router.route(request).await,
        }
    }
}
