use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

use crate::dto::schema::UserName;
use crate::dto::schema::UserNameRaw;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct RegistrationWithPasswordSchema {
    #[serde(rename = "name")]
    pub name: UserName,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "terms")]
    pub terms: bool,
}

impl RegistrationWithPasswordSchema {
    pub fn new(name: UserName, email: String, password: String, terms: bool) -> Self {
        Self {
            name,
            email,
            password,
            terms,
        }
    }
}

impl SerializableBody for RegistrationWithPasswordSchema {}

impl DeserializableSchemaRaw<'_> for RegistrationWithPasswordSchema {
    type Raw = RegistrationWithPasswordSchemaRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

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
        let email = raw.email;
        if email.is_none() {
            argentum_violations.insert(
                "email".into(),
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
        let terms = raw.terms;
        if terms.is_none() {
            argentum_violations.insert(
                "terms".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }

        if argentum_violations.is_empty()
            && let Some(name) = name
            && let Some(email) = email
            && let Some(password) = password
            && let Some(terms) = terms
        {
            Ok(Self::new(name, email, password, terms))
        } else {
            Err(Violations::new(
                vec!["wrong data for RegistrationWithPasswordSchema".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct RegistrationWithPasswordSchemaRaw {
    #[serde(rename = "name")]
    pub name: Option<UserNameRaw>,
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "password")]
    pub password: Option<String>,
    #[serde(rename = "terms")]
    pub terms: Option<bool>,
}
