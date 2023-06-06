use crate::data_type::RequestTrait;
use crate::service::ValidationErrorTransformer;
use argentum_standard_business::invariant_violation::{InvariantResult, Violations};
use serde::Deserialize;
use serde_valid::json::FromJsonSlice;
use std::sync::Arc;

pub struct SchemaExtractor {
    validation_error_transformer: Arc<ValidationErrorTransformer>,
}

impl SchemaExtractor {
    pub fn new(validation_error_transformer: Arc<ValidationErrorTransformer>) -> Self {
        Self {
            validation_error_transformer,
        }
    }

    pub async fn extract<B>(&self, request: impl RequestTrait) -> InvariantResult<B>
    where
        B: for<'a> Deserialize<'a> + for<'a> FromJsonSlice<'a>,
    {
        let result = request.fetch_body().await;

        if result.is_err() {
            //TODO: log error
            return Err(Violations::new(
                vec!["Can't receive request body".to_string()],
                None,
            ));
        }

        let mut body = result.unwrap();

        if body.is_empty() {
            body = Vec::from("{}");
        }

        let deserialized = B::from_json_slice(&body);

        match deserialized {
            Ok(value) => Ok(value),
            Err(e) => {
                let violations = self.validation_error_transformer.transform(e);

                Err(violations)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::RequestTrait;
    use crate::service::{SchemaExtractor, ValidationErrorTransformer};
    use async_trait::async_trait;
    use hyper::{Error, HeaderMap, Method};
    use serde::Deserialize;
    use serde_valid::Validate;
    use std::sync::Arc;

    #[derive(Debug, Deserialize, Validate)]
    struct EmptyRequestMock {}

    struct RequestMock {}

    #[async_trait]
    impl RequestTrait for RequestMock {
        async fn fetch_body(self) -> Result<Vec<u8>, Error> {
            Ok(vec![])
        }

        fn method(&self) -> &Method {
            &Method::OPTIONS
        }

        fn get_headers(&self) -> &HeaderMap {
            todo!()
        }
    }

    struct RequestWithBadJsonMock {}

    #[async_trait]
    impl RequestTrait for RequestWithBadJsonMock {
        async fn fetch_body(self) -> Result<Vec<u8>, Error> {
            Ok(Vec::from(",;/"))
        }

        fn method(&self) -> &Method {
            &Method::OPTIONS
        }

        fn get_headers(&self) -> &HeaderMap {
            todo!()
        }
    }

    #[tokio::test]
    pub async fn test_extract() {
        let extractor = SchemaExtractor::new(Arc::new(ValidationErrorTransformer::new()));

        let req = RequestMock {};
        let result = extractor.extract::<EmptyRequestMock>(req).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    pub async fn test_extract_bad_json() {
        let extractor = SchemaExtractor::new(Arc::new(ValidationErrorTransformer::new()));

        let req = RequestWithBadJsonMock {};
        let result = extractor.extract::<EmptyRequestMock>(req).await;

        assert!(result.is_err());
    }
}
