use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GuestRegistrationResult {
    pub guest_id: uuid::Uuid,
    pub token: String,
}

impl GuestRegistrationResult {
    pub fn new(guest_id: uuid::Uuid, token: String) -> Self {
        Self { guest_id, token }
    }
}

impl SerializableBody for GuestRegistrationResult {}

impl DeserializableSchemaRaw<'_> for GuestRegistrationResult {
    type Raw = GuestRegistrationResultRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let guest_id = raw.guest_id;
        if guest_id.is_none() {
            argentum_violations.insert(
                "guest_id".into(),
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
            && let Some(guest_id) = guest_id
            && let Some(token) = token
        {
            Ok(Self::new(guest_id, token))
        } else {
            Err(Violations::new(
                vec!["wrong data for GuestRegistrationResult".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct GuestRegistrationResultRaw {
    #[serde(rename = "guest_id")]
    pub guest_id: Option<uuid::Uuid>,
    #[serde(rename = "token")]
    pub token: Option<String>,
}
