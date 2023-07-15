use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_rest_infrastructure::service::DeserializableSchemaRaw;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct RequestRestoreTokenSchema {
    pub email: String,
}

impl RequestRestoreTokenSchema {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}

impl SerializableBody for RequestRestoreTokenSchema {}

impl DeserializableSchemaRaw<'_> for RequestRestoreTokenSchema {
    type Raw = RequestRestoreTokenSchemaRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let email = raw.email;
        if email.is_none() {
            argentum_violations.insert(
                "email".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }

        if argentum_violations.is_empty() {
            Ok(Self::new(email.unwrap()))
        } else {
            Err(Violations::new(
                vec!["wrong data for RequestRestoreTokenSchema".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct RequestRestoreTokenSchemaRaw {
    #[serde(rename = "email")]
    pub email: Option<String>,
}
