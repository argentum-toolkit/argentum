use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Violation {
    pub errors: Option<String>,

    pub items: Option<String>,
}

impl Violation {
    pub fn new(errors: Option<String>, items: Option<String>) -> Self {
        Self { errors, items }
    }
}

impl SerializableBody for Violation {}

impl DeserializableSchemaRaw<'_> for Violation {
    type Raw = ViolationRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let errors = raw.errors;
        let items = raw.items;

        if argentum_violations.is_empty() {
            Ok(Self::new(errors, items))
        } else {
            Err(Violations::new(
                vec!["wrong data for Violation".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ViolationRaw {
    #[serde(rename = "errors")]
    pub errors: Option<String>,
    #[serde(rename = "items")]
    pub items: Option<String>,
}
