use crate::service::ValidationErrorTransformer;
use argentum_standard_business::invariant_violation::InvariantResult;
use hyper::HeaderMap;
use serde::Deserialize;
use serde_valid::json::FromJsonSlice;
use std::collections::HashMap;
use std::sync::Arc;

pub struct HeaderParamsExtractor {
    validation_error_transformer: Arc<ValidationErrorTransformer>,
}

impl HeaderParamsExtractor {
    pub fn new(validation_error_transformer: Arc<ValidationErrorTransformer>) -> Self {
        Self {
            validation_error_transformer,
        }
    }

    pub fn extract<R>(&self, headers: HeaderMap) -> InvariantResult<R>
    where
        R: for<'a> Deserialize<'a> + for<'a> FromJsonSlice<'a>,
    {
        let mut map = HashMap::new();

        for (k, v) in headers.into_iter() {
            if let Some(key) = k {
                map.insert(key.to_string(), v.to_str().unwrap().to_string());
            }
        }
        let pp = serde_json::to_string(&map).unwrap();

        let deserialized = R::from_json_slice(pp.as_ref());

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
    use crate::service::{PathParamsExtractor, ValidationErrorTransformer};
    use serde::Deserialize;
    use serde_valid::Validate;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[derive(Debug, Deserialize, Validate)]
    struct ExtractMock {
        pub title: String,
    }

    #[test]
    pub fn test_extract() {
        let extractor = PathParamsExtractor::new(Arc::new(ValidationErrorTransformer::new()));

        let params = HashMap::from([("title".to_string(), "v".to_string())]);
        let result = extractor.extract::<ExtractMock>(params);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "v".to_string());
    }

    #[test]
    pub fn test_extract_with_wrong_params() {
        let extractor = PathParamsExtractor::new(Arc::new(ValidationErrorTransformer::new()));

        let params = HashMap::from([("wrong_field".to_string(), "v".to_string())]);
        let result = extractor.extract::<ExtractMock>(params);

        assert!(result.is_err());
    }
}
