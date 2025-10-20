use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct ChangePasswordSchema {
    pub token: String,
    pub password: String,
}

impl ChangePasswordSchema {
    pub fn new(token: String, password: String) -> Self {
        Self { token, password }
    }
}

impl SerializableBody for ChangePasswordSchema {}

impl DeserializableSchemaRaw<'_> for ChangePasswordSchema {
    type Raw = ChangePasswordSchemaRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let token = raw.token;
        if token.is_none() {
            argentum_violations.insert(
                "token".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let password = raw.password;
        if password.is_none() {
            argentum_violations.insert(
                "password".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }

        if argentum_violations.is_empty()
            && let Some(token) = token
            && let Some(password) = password
        {
            Ok(Self::new(token, password))
        } else {
            Err(Violations::new(
                vec!["wrong data for ChangePasswordSchema".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ChangePasswordSchemaRaw {
    #[serde(rename = "token")]
    pub token: Option<String>,
    #[serde(rename = "password")]
    pub password: Option<String>,
}
