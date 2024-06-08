use async_trait::async_trait;

use crate::data_type::error::HttpError;
use crate::data_type::{HttpResponse, Request};
use hyper::{Method, Uri};

#[async_trait]
pub trait Router: Send + Sync {
    fn is_route_supported(&self, uri: &Uri, method: &Method) -> bool;
    async fn route(&self, request: Request) -> Result<HttpResponse, HttpError>;
}
