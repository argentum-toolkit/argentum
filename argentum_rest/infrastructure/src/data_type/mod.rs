mod deserializable_schema_raw;
pub mod error;
mod http_request;
pub mod http_response;
mod problem_detail;

use bytes::Bytes;
use http_body_util::Full;
pub use http_response::EmptyBody;
use hyper::body::Incoming as IncomingBody;
use hyper::{http, Error, HeaderMap, Method};

pub use deserializable_schema_raw::DeserializableSchemaRaw;
pub use http_response::HttpResponse;
pub use http_response::SerializableBody;

pub type Request = http::Request<IncomingBody>;
pub type Response = http::Response<Full<Bytes>>;

use async_trait::async_trait;
use http_body_util::BodyExt;

#[async_trait]
pub trait RequestTrait {
    async fn fetch_body(self) -> Result<Vec<u8>, hyper::Error>;
    fn method(&self) -> &Method;
    fn get_headers(&self) -> &HeaderMap;
}

#[async_trait]
impl RequestTrait for Request {
    async fn fetch_body(self) -> Result<Vec<u8>, Error> {
        Ok(self.into_body().collect().await?.to_bytes().to_vec())
    }

    fn method(&self) -> &Method {
        self.method()
    }

    fn get_headers(&self) -> &HeaderMap {
        self.headers()
    }
}

pub use http_request::AuthHeaderParams;
pub use http_request::EmptyHeaderParams;
pub use http_request::EmptyPathParams;
pub use http_request::EmptyQueryParams;
pub use http_request::EmptyRequestBody;
pub use http_request::HttpHeaderParams;
pub use http_request::HttpParams;
pub use http_request::HttpPathParams;
pub use http_request::HttpQueryParams;
pub use http_request::HttpRequest;
pub use problem_detail::ProblemDetail;
pub use problem_detail::ProblemDetailExtension;
