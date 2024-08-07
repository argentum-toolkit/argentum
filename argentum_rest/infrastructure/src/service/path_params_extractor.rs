use crate::service::{RawPathParams, ValidationErrorTransformer};
use argentum_standard_business::invariant_violation::InvariantResult;
use serde::Deserialize;
use serde_valid::json::FromJsonSlice;
use std::sync::Arc;

pub struct PathParamsExtractor {
    validation_error_transformer: Arc<ValidationErrorTransformer>,
}

impl PathParamsExtractor {
    pub fn new(validation_error_transformer: Arc<ValidationErrorTransformer>) -> Self {
        Self {
            validation_error_transformer,
        }
    }

    pub fn extract<R>(&self, raw_path_params: RawPathParams) -> InvariantResult<R>
    where
        R: for<'a> Deserialize<'a> + for<'a> FromJsonSlice<'a>,
    {
        let pp = serde_json::to_string(&raw_path_params).unwrap();
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

        let params = HashMap::from([("title", "v")]);
        let result = extractor.extract::<ExtractMock>(params);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "v".to_string());
    }

    #[test]
    pub fn test_extract_with_wrong_params() {
        let extractor = PathParamsExtractor::new(Arc::new(ValidationErrorTransformer::new()));

        let params = HashMap::from([("wrong_field", "v")]);
        let result = extractor.extract::<ExtractMock>(params);

        assert!(result.is_err());
    }
}
