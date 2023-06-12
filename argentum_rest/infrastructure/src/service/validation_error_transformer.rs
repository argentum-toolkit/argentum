use argentum_standard_business::invariant_violation::ViolationItem;
use argentum_standard_business::invariant_violation::Violations;
use serde_valid::validation::{Errors, VecErrors};
use std::collections::HashMap;

#[derive(Default)]
pub struct ValidationErrorTransformer {}

impl ValidationErrorTransformer {
    pub fn new() -> Self {
        Self {}
    }

    fn collect_errors<E>(&self, errors: VecErrors<E>) -> Vec<String>
    where
        E: 'static + std::error::Error,
    {
        let mut res = vec![];
        for err in errors.into_iter() {
            res.push(err.to_string())
        }

        res
    }

    fn collect_violations<E>(&self, e: Errors<E>) -> Violations
    where
        E: 'static + std::error::Error,
    {
        match e {
            Errors::Array(e) => {
                let errors = self.collect_errors(e.errors);

                let mut items_vec = vec![];

                for (_, values) in e.items {
                    items_vec.push(self.collect_violations(values));
                }

                let items = if !items_vec.is_empty() {
                    Some(ViolationItem::Array(items_vec))
                } else {
                    None
                };
                Violations::new(errors, items)
            }
            Errors::Object(e) => {
                let errors = self.collect_errors(e.errors);
                let mut properties = HashMap::new();

                for (name, values) in e.properties {
                    properties.insert(name.to_string(), self.collect_violations(values));
                }

                let items = if properties.is_empty() {
                    None
                } else {
                    Some(ViolationItem::Object(properties))
                };
                Violations::new(errors, items)
            }
            Errors::NewType(e) => {
                let errors = self.collect_errors(e);

                Violations::new(errors, None)
            }
        }
    }

    pub fn transform<E>(&self, e: serde_valid::Error<E>) -> Violations
    where
        E: 'static + std::error::Error,
    {
        match e {
            serde_valid::Error::DeserializeError(e) => Violations::new(vec![e.to_string()], None),
            serde_valid::Error::ValidationError(e) => self.collect_violations(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::service::ValidationErrorTransformer;
    use argentum_standard_business::invariant_violation::ViolationItem;
    use serde_valid::validation::{ArrayErrors, ItemErrorsMap, ObjectErrors, PropertyErrorsMap};

    #[derive(thiserror::Error, Debug)]
    enum MockError {
        #[error("Deserialization error")]
        Deserialization,
    }

    #[test]
    fn test_deserialization_error() {
        let transformer = ValidationErrorTransformer::new();
        let e = serde_valid::Error::DeserializeError(MockError::Deserialization);
        let v = transformer.transform(e);

        assert!(!v.is_empty());
        assert!(!v.errors.is_empty());
        assert!(v.items.is_none());
        assert_eq!(v.errors.len(), 1);
        assert_eq!(v.errors.first().unwrap(), "Deserialization error")
    }

    #[test]
    fn test_validation_error_with_array_items() {
        let transformer = ValidationErrorTransformer::new();

        let e: serde_valid::Error<MockError> = serde_valid::Error::ValidationError(
            serde_valid::validation::Errors::Array(ArrayErrors::new(
                vec![serde_valid::validation::Error::Custom(
                    "Some err".to_string(),
                )],
                ItemErrorsMap::new(),
            )),
        );
        let v = transformer.transform(e);

        assert!(!v.is_empty());
        assert!(!v.errors.is_empty());
        assert_eq!(v.errors.len(), 1);
        assert_eq!(v.errors.first().unwrap(), "Some err");
        assert!(v.items.is_none());
    }

    #[test]
    fn test_validation_error_with_array_items_tree() {
        let transformer = ValidationErrorTransformer::new();

        let e: serde_valid::Error<MockError> = serde_valid::Error::ValidationError(
            serde_valid::validation::Errors::Array(ArrayErrors::new(
                vec![],
                ItemErrorsMap::from([(
                    1,
                    serde_valid::validation::Errors::Array(ArrayErrors::new(
                        vec![serde_valid::validation::Error::Custom(
                            "Some err".to_string(),
                        )],
                        ItemErrorsMap::new(),
                    )),
                )]),
            )),
        );

        let v = transformer.transform(e);

        assert!(!v.is_empty());
        assert!(v.errors.is_empty());
        assert!(v.items.is_some());
        match v.items.unwrap() {
            ViolationItem::Object(_) => {
                panic!("Should be an array")
            }
            ViolationItem::Array(a) => {
                assert_eq!(a.len(), 1);
                assert_eq!(a.first().unwrap().errors.first().unwrap(), "Some err");
                assert!(a.first().unwrap().items.is_none());
            }
        }
    }

    #[test]
    fn test_validation_error_with_object_items() {
        let transformer = ValidationErrorTransformer::new();

        let e: serde_valid::Error<MockError> = serde_valid::Error::ValidationError(
            serde_valid::validation::Errors::Object(ObjectErrors::new(
                vec![serde_valid::validation::Error::Custom(
                    "Some err".to_string(),
                )],
                PropertyErrorsMap::new(),
            )),
        );
        let v = transformer.transform(e);

        assert!(!v.is_empty());
        assert!(!v.errors.is_empty());
        assert_eq!(v.errors.len(), 1);
        assert_eq!(v.errors.first().unwrap(), "Some err");
        assert!(v.items.is_none());
    }

    #[test]
    fn test_validation_error_with_object_items_tree() {
        let transformer = ValidationErrorTransformer::new();

        let e: serde_valid::Error<MockError> = serde_valid::Error::ValidationError(
            serde_valid::validation::Errors::Object(ObjectErrors::new(
                vec![],
                PropertyErrorsMap::from([(
                    "some-field".to_string(),
                    serde_valid::validation::Errors::Object(ObjectErrors::new(
                        vec![serde_valid::validation::Error::Custom(
                            "Some err".to_string(),
                        )],
                        PropertyErrorsMap::new(),
                    )),
                )]),
            )),
        );

        let v = transformer.transform(e);

        assert!(!v.is_empty());
        assert!(v.errors.is_empty());
        assert!(v.items.is_some());
        match v.items.unwrap() {
            ViolationItem::Object(o) => {
                assert_eq!(o.len(), 1);
                assert_eq!(
                    o.get("some-field").unwrap().errors.first().unwrap(),
                    "Some err"
                );
                assert!(o.get("some-field").unwrap().items.is_none());
            }
            ViolationItem::Array(_) => {
                panic!("Should be an object")
            }
        }
    }

    #[test]
    fn test_validation_error_with_new_type_items() {
        let transformer = ValidationErrorTransformer::new();

        let e: serde_valid::Error<MockError> =
            serde_valid::Error::ValidationError(serde_valid::validation::Errors::NewType(vec![
                serde_valid::validation::Error::Custom("Some err".to_string()),
            ]));
        let v = transformer.transform(e);

        assert!(!v.is_empty());
        assert!(!v.errors.is_empty());
        assert_eq!(v.errors.len(), 1);
        assert_eq!(v.errors.first().unwrap(), "Some err");
        assert!(v.items.is_none());
    }
}
