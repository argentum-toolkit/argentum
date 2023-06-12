use crate::data_type::{ProblemDetailExtension, SerializableBody};
use argentum_standard_business::invariant_violation::Violations;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct BadRequestError {
    body: Violations,
    path: Violations,
    headers: Violations,
}

impl Serialize for BadRequestError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BadRequestError", 2)?;

        if !self.body.is_empty() {
            state.serialize_field("body", &ViolationsDto::from(&self.body))?;
        }

        if !self.path.is_empty() {
            state.serialize_field("path", &ViolationsDto::from(&self.path))?;
        }

        if !self.headers.is_empty() {
            state.serialize_field("headers", &ViolationsDto::from(&self.headers))?;
        }

        state.end()
    }
}

impl BadRequestError {
    pub fn new(body: Violations, path: Violations, headers: Violations) -> Self {
        Self {
            body,
            path,
            headers,
        }
    }
}

impl SerializableBody for BadRequestError {}

impl ProblemDetailExtension for BadRequestError {}

#[cfg(test)]
mod tests {
    use crate::data_type::error::BadRequestError;
    use argentum_standard_business::invariant_violation::ViolationItem;
    use argentum_standard_business::invariant_violation::Violations;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn empty_bad_request_serialize() {
        let br = BadRequestError::new(
            Violations::new(vec![], None),
            Violations::new(vec![], None),
            Violations::new(vec![], None),
        );

        let str = serde_json::to_string(&br).unwrap();

        assert_eq!(true, br.body.is_empty());
        assert_eq!(true, br.path.is_empty());
        assert_eq!("{}".to_string(), str);
    }

    #[test]
    fn full_bad_request_serialize() {
        let br = BadRequestError::new(
            Violations::new(
                vec!["error1.1".to_string(), "error1.2".to_string()],
                Some(ViolationItem::Array(vec![Violations::new(
                    vec![],
                    Some(ViolationItem::Object(HashMap::from([(
                        "str".to_string(),
                        Violations::new(vec!["error1.3".to_string()], None),
                    )]))),
                )])),
            ),
            Violations::new(
                vec!["error2.1".to_string(), "error2.2".to_string()],
                Some(ViolationItem::Array(vec![Violations::new(
                    vec!["error2.3".to_string()],
                    None,
                )])),
            ),
            Violations::new(vec![], None),
        );

        let str = serde_json::to_value(&br).unwrap();

        let expected = json!({
            "body": {
                "errors": [
                    "error1.1",
                    "error1.2"
                ],
                "items": [
                    {
                        "errors": [],
                        "items": {
                            "str": {
                                "errors": [
                                    "error1.3"
                                ],
                                "items": null
                            }
                        }
                    }
                ]
            },
            "path": {
                "errors": [
                    "error2.1",
                    "error2.2"
                ],
                "items": [
                    {
                        "errors": [
                            "error2.3"
                        ],
                        "items": null
                    }
                ]
            }
        });

        assert_eq!(false, br.body.is_empty());
        assert_eq!(false, br.path.is_empty());
        assert_eq!(str, expected);
    }
}
