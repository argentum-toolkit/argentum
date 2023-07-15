use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct AnonymousRegistrationResult {
    pub aonymous_id: String,

    pub token: String,
}

impl AnonymousRegistrationResult {
    pub fn new(aonymous_id: String, token: String) -> Self {
        Self { aonymous_id, token }
    }
}

impl SerializableBody for AnonymousRegistrationResult {}

impl DeserializableSchemaRaw<'_> for AnonymousRegistrationResult {
    type Raw = AnonymousRegistrationResultRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let aonymous_id = raw.aonymous_id;
        if aonymous_id.is_none() {
            argentum_violations.insert(
                "aonymous_id".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let token = raw.token;
        if token.is_none() {
            argentum_violations.insert(
                "token".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }

        if argentum_violations.is_empty() {
            Ok(Self::new(aonymous_id.unwrap(), token.unwrap()))
        } else {
            Err(Violations::new(
                vec!["wrong data for AnonymousRegistrationResult".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct AnonymousRegistrationResultRaw {
    #[serde(rename = "aonymous_id")]
    pub aonymous_id: Option<String>,
    #[serde(rename = "token")]
    pub token: Option<String>,
}
