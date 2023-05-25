use crate::data_type::{HttpResponse, Response};
use bytes::Bytes;
use http_body_util::Full;

pub struct ResponseToJsonTransformer {}

impl ResponseToJsonTransformer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(&self, response: HttpResponse) -> Response {
        let body = serde_json::to_vec_pretty(&response.body).unwrap();
        let body = Full::new(Bytes::from(body));

        hyper::Response::builder()
            .status(response.code)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            // .header(hyper::header::CONTENT_ENCODING, "deflate")
            .body(body)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::{EmptyBody, HttpResponse};
    use crate::service::ResponseToJsonTransformer;
    use hyper::StatusCode;

    #[test]
    fn test_transform() {
        let transformer = ResponseToJsonTransformer::new();

        let response = HttpResponse::new(StatusCode::CREATED, EmptyBody::new_boxed());

        let hyper_response = transformer.transform(response);

        assert_eq!(hyper_response.status(), StatusCode::CREATED);
        assert_eq!(
            hyper_response
                .headers()
                .get(hyper::header::CONTENT_TYPE)
                .unwrap(),
            "application/json"
        );
    }
}
