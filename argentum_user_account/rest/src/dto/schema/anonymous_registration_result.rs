use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;


#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct AnonymousRegistrationResult {
    pub anonymous_id: uuid::Uuid,

    pub token: String,

}

impl AnonymousRegistrationResult {
    pub fn new(
        anonymous_id: uuid::Uuid,
        token: String,
    ) -> Self {
        Self {
            anonymous_id,
            token,
        }
    }
}

impl SerializableBody for AnonymousRegistrationResult {}

impl DeserializableSchemaRaw<'_> for AnonymousRegistrationResult {
    type Raw = AnonymousRegistrationResultRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

                let anonymous_id = raw.anonymous_id;
                    if anonymous_id.is_none() {
                        argentum_violations.insert(
                            "anonymous_id".into(),
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
            Ok(Self::new(
                anonymous_id.unwrap(),
                token.unwrap(),
            ))
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
    #[serde(rename = "anonymous_id")]
    pub anonymous_id: Option<uuid::Uuid>,
    #[serde(rename = "token")]
    pub token: Option<String>,
}
