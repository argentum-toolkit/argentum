use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

use crate::dto::schema::UserName;
use crate::dto::schema::UserNameRaw;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct User {
    pub email: String,

    pub id: Option<uuid::Uuid>,

    pub name: UserName,
}

impl User {
    pub fn new(email: String, id: Option<uuid::Uuid>, name: UserName) -> Self {
        Self { email, id, name }
    }
}

impl SerializableBody for User {}

impl DeserializableSchemaRaw<'_> for User {
    type Raw = UserRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let email = raw.email;
        if email.is_none() {
            argentum_violations.insert(
                "email".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let id = raw.id;
        let name = if raw.name.is_none() {
            argentum_violations.insert(
                "name".into(),
                Violations::new(vec!["required field".to_string()], None),
            );
            None
        } else {
            match UserName::try_from_raw(raw.name.unwrap()) {
                Ok(value) => Some(value),
                Err(v) => {
                    argentum_violations.insert("name".into(), v);

                    None
                }
            }
        };

        if argentum_violations.is_empty() {
            Ok(Self::new(email.unwrap(), id, name.unwrap()))
        } else {
            Err(Violations::new(
                vec!["wrong data for User".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct UserRaw {
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "id")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "name")]
    pub name: Option<UserNameRaw>,
}
