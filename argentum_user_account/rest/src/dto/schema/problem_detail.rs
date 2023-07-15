use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ProblemDetail {
    pub code: String,

    pub message: Option<String>,
}

impl ProblemDetail {
    pub fn new(code: String, message: Option<String>) -> Self {
        Self { code, message }
    }
}

impl SerializableBody for ProblemDetail {}

impl DeserializableSchemaRaw<'_> for ProblemDetail {
    type Raw = ProblemDetailRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let code = raw.code;
        if code.is_none() {
            argentum_violations.insert(
                "code".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let message = raw.message;

        if argentum_violations.is_empty() {
            Ok(Self::new(code.unwrap(), message))
        } else {
            Err(Violations::new(
                vec!["wrong data for ProblemDetail".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ProblemDetailRaw {
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[serde(rename = "message")]
    pub message: Option<String>,
}
