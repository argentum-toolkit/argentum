use crate::data_type::RequestTrait;
use argentum_standard_business::invariant_violation::{InvariantResult, Violations};
use serde::Deserialize;

#[derive(Default)]
pub struct SchemaExtractor {}

//TODO: move to separated file
pub trait DeserializableSchemaRaw<'a>: Sized {
    type Raw: Deserialize<'a>;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self>;
}

impl SchemaExtractor {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn extract<B>(&self, request: impl RequestTrait) -> InvariantResult<B>
    where
        B: for<'a> DeserializableSchemaRaw<'a>,
    {
        let result = request.fetch_body().await;

        if result.is_err() {
            return Err(Violations::new(
                vec!["Can't receive request body".to_string()],
                None,
            ));
        }

        let mut body = result.unwrap();

        if body.is_empty() {
            body = Vec::from("{}");
        }

        // let deserialized = B::from_json_slice(&body);
        let deserialized: serde_json::Result<B::Raw> = serde_json::from_slice(&body);

        match deserialized {
            Ok(raw) => B::try_from_raw(raw),
            Err(e) => Err(Violations::new(vec![e.to_string()], None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::RequestTrait;
    use crate::service::{DeserializableSchemaRaw, SchemaExtractor};
    use argentum_standard_business::invariant_violation::InvariantResult;
    use async_trait::async_trait;
    use hyper::{Error, HeaderMap, Method};
    use serde_valid::Validate;
    use std::collections::HashMap;

    #[derive(Debug, Validate)]
    struct EmptyRequestMock {}
    impl DeserializableSchemaRaw<'_> for EmptyRequestMock {
        type Raw = HashMap<String, String>;

        fn try_from_raw(_: Self::Raw) -> InvariantResult<Self> {
            Ok(Self {})
        }
    }

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
        let extractor = SchemaExtractor::new();

        let req = RequestMock {};
        let result = extractor.extract::<EmptyRequestMock>(req).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    pub async fn test_extract_bad_json() {
        let extractor = SchemaExtractor::new();

        let req = RequestWithBadJsonMock {};
        let result = extractor.extract::<EmptyRequestMock>(req).await;

        assert!(result.is_err());
    }
}
