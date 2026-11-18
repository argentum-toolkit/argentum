use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteAccountSchema {
    #[serde(rename = "password")]
    pub password: String,
}

impl DeleteAccountSchema {
    pub fn new(password: String) -> Self {
        Self { password }
    }
}

impl SerializableBody for DeleteAccountSchema {}

impl DeserializableSchemaRaw<'_> for DeleteAccountSchema {
    type Raw = DeleteAccountSchemaRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let password = raw.password;
        if password.is_none() {
            argentum_violations.insert(
                "password".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }

        if argentum_violations.is_empty()
            && let Some(password) = password
        {
            Ok(Self::new(password))
        } else {
            Err(Violations::new(
                vec!["wrong data for DeleteAccountSchema".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct DeleteAccountSchemaRaw {
    #[serde(rename = "password")]
    pub password: Option<String>,
}
