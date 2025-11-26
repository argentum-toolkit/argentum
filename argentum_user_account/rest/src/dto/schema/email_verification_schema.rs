use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct EmailVerificationSchema {
    pub key: String,
}

impl EmailVerificationSchema {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl SerializableBody for EmailVerificationSchema {}

impl DeserializableSchemaRaw<'_> for EmailVerificationSchema {
    type Raw = EmailVerificationSchemaRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let key = raw.key;
        if key.is_none() {
            argentum_violations.insert(
                "key".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }

        if argentum_violations.is_empty()
            && let Some(key) = key
        {
            Ok(Self::new(key))
        } else {
            Err(Violations::new(
                vec!["wrong data for EmailVerificationSchema".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct EmailVerificationSchemaRaw {
    #[serde(rename = "key")]
    pub key: Option<String>,
}
