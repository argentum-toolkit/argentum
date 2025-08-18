use crate::data_type::{HttpResponse, Response};
use bytes::Bytes;
use http_body_util::Full;

#[derive(Default)]
pub struct ResponseToJsonTransformer {}

impl ResponseToJsonTransformer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(&self, response: HttpResponse) -> Result<Response, String> {
        let body = serde_json::to_vec_pretty(&response.body)
            .map_err(|e| format!("Can't serialize response body. Error: {e}"))?;

        let body = Full::new(Bytes::from(body));

        hyper::Response::builder()
            .status(response.code)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            // .header(hyper::header::CONTENT_ENCODING, "deflate")
            .body(body)
            .map_err(|e| format!("Can't build response body. Err: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use http::StatusCode;

    use crate::data_type::{EmptyBody, HttpResponse};
    use crate::service::ResponseToJsonTransformer;

    #[test]
    fn test_transform() {
        let transformer = ResponseToJsonTransformer::new();

        let response = HttpResponse::new(StatusCode::CREATED, EmptyBody::new_boxed());

        let hyper_response = transformer
            .transform(response)
            .expect("should be able to transform response");

        assert_eq!(hyper_response.status().as_u16(), StatusCode::CREATED);
        assert_eq!(
            hyper_response
                .headers()
                .get(hyper::header::CONTENT_TYPE)
                .expect("Content-Type header is not found"),
            "application/json"
        );
    }
}
