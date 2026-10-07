use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct ExchangeCodeResult {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "userId")]
    pub user_id: uuid::Uuid,
}

impl ExchangeCodeResult {
    pub fn new(access_token: String, user_id: uuid::Uuid) -> Self {
        Self {
            access_token,
            user_id,
        }
    }
}

impl SerializableBody for ExchangeCodeResult {}

impl DeserializableSchemaRaw<'_> for ExchangeCodeResult {
    type Raw = ExchangeCodeResultRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let access_token = raw.access_token;
        if access_token.is_none() {
            argentum_violations.insert(
                "access_token".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let user_id = raw.user_id;
        if user_id.is_none() {
            argentum_violations.insert(
                "user_id".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }

        if argentum_violations.is_empty()
            && let Some(access_token) = access_token
            && let Some(user_id) = user_id
        {
            Ok(Self::new(access_token, user_id))
        } else {
            Err(Violations::new(
                vec!["wrong data for ExchangeCodeResult".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ExchangeCodeResultRaw {
    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<uuid::Uuid>,
}
