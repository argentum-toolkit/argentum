use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct LoginResult {
    pub user_id: uuid::Uuid,
    pub token: String,
}

impl LoginResult {
    pub fn new(user_id: uuid::Uuid, token: String) -> Self {
        Self { user_id, token }
    }
}

impl SerializableBody for LoginResult {}

impl DeserializableSchemaRaw<'_> for LoginResult {
    type Raw = LoginResultRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let user_id = raw.user_id;
        if user_id.is_none() {
            argentum_violations.insert(
                "user_id".into(),
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

        if argentum_violations.is_empty()
            && let Some(user_id) = user_id
            && let Some(token) = token
        {
            Ok(Self::new(user_id, token))
        } else {
            Err(Violations::new(
                vec!["wrong data for LoginResult".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct LoginResultRaw {
    #[serde(rename = "user_id")]
    pub user_id: Option<uuid::Uuid>,
    #[serde(rename = "token")]
    pub token: Option<String>,
}
