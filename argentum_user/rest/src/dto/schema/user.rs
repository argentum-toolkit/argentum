use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

use crate::dto::schema::UserName;
use crate::dto::schema::UserNameRaw;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct User {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "name")]
    pub name: UserName,
}

impl User {
    pub fn new(id: uuid::Uuid, slug: String, email: String, name: UserName) -> Self {
        Self {
            id,
            slug,
            email,
            name,
        }
    }
}

impl SerializableBody for User {}

impl DeserializableSchemaRaw<'_> for User {
    type Raw = UserRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let id = raw.id;
        if id.is_none() {
            argentum_violations.insert(
                "id".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let slug = raw.slug;
        if slug.is_none() {
            argentum_violations.insert(
                "slug".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let email = raw.email;
        if email.is_none() {
            argentum_violations.insert(
                "email".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let name = match raw.name {
            Some(raw_name) => match UserName::try_from_raw(raw_name) {
                Ok(value) => Some(value),
                Err(v) => {
                    argentum_violations.insert("name".into(), v);

                    None
                }
            },
            None => {
                argentum_violations.insert(
                    "name".into(),
                    Violations::new(vec!["required field".to_string()], None),
                );
                None
            }
        };

        if argentum_violations.is_empty()
            && let Some(id) = id
            && let Some(slug) = slug
            && let Some(email) = email
            && let Some(name) = name
        {
            Ok(Self::new(id, slug, email, name))
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
    #[serde(rename = "id")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "slug")]
    pub slug: Option<String>,
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<UserNameRaw>,
}
