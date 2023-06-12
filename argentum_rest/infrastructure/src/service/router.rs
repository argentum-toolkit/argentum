use async_trait::async_trait;

use crate::data_type::error::HttpError;
use crate::data_type::{HttpResponse, Request};

#[async_trait]
pub trait Router: Send + Sync {
    async fn route(&self, request: Request) -> Result<HttpResponse, HttpError>;
}
