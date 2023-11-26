use hyper::StatusCode;
use serde::Serialize;

pub trait SerializableBody: erased_serde::Serialize {}
erased_serde::serialize_trait_object!(SerializableBody);

pub trait ContentTypeResponseTrait {
    fn content_type(&self) -> Option<String>;
    fn body(&self) -> Box<dyn SerializableBody>;
}

#[derive(Clone)]
pub struct NoContent {}

impl ContentTypeResponseTrait for NoContent {
    fn content_type(&self) -> Option<String> {
        None
    }

    fn body(&self) -> Box<dyn SerializableBody> {
        EmptyBody::new_boxed()
    }
}

impl NoContent {
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

// #[derive(Serialize)]
pub struct HttpResponse {
    pub code: StatusCode,

    pub body: Box<dyn SerializableBody>,
    //TODO: headers
}

impl HttpResponse {
    pub fn new(code: StatusCode, body: Box<dyn SerializableBody>) -> Self {
        HttpResponse { code, body }
    }
}

#[derive(Debug, Serialize)]
pub struct EmptyBody {}

impl SerializableBody for EmptyBody {}

impl EmptyBody {
    pub fn new_boxed() -> Box<EmptyBody> {
        Box::new(EmptyBody {})
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::{EmptyBody, HttpResponse};
    use hyper::StatusCode;

    #[test]
    fn test_constructor() {
        let teapot_response = HttpResponse::new(StatusCode::IM_A_TEAPOT, EmptyBody::new_boxed());
        assert_eq!(StatusCode::IM_A_TEAPOT, teapot_response.code);

        let ok_response = HttpResponse::new(StatusCode::OK, EmptyBody::new_boxed());
        assert_eq!(StatusCode::OK, ok_response.code);
    }
}
