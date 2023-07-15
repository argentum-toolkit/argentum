use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct RegistrationWithPasswordResult {
    pub id: String,
}

impl RegistrationWithPasswordResult {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl SerializableBody for RegistrationWithPasswordResult {}

impl DeserializableSchemaRaw<'_> for RegistrationWithPasswordResult {
    type Raw = RegistrationWithPasswordResultRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let id = raw.id;
        if id.is_none() {
            argentum_violations.insert(
                "id".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }

        if argentum_violations.is_empty() {
            Ok(Self::new(id.unwrap()))
        } else {
            Err(Violations::new(
                vec!["wrong data for RegistrationWithPasswordResult".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct RegistrationWithPasswordResultRaw {
    #[serde(rename = "id")]
    pub id: Option<String>,
}
