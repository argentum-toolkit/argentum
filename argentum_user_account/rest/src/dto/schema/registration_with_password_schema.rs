use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

use crate::dto::schema::UserName;
use crate::dto::schema::UserNameRaw;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct RegistrationWithPasswordSchema {
    pub email: String,

    pub name: UserName,

    pub password: String,

}

impl RegistrationWithPasswordSchema {
    pub fn new(
        email: String,
        name: UserName,
        password: String,
    ) -> Self {
        Self {
            email,
            name,
            password,
        }
    }
}

impl SerializableBody for RegistrationWithPasswordSchema {}

impl DeserializableSchemaRaw<'_> for RegistrationWithPasswordSchema {
    type Raw = RegistrationWithPasswordSchemaRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

                let email = raw.email;
                    if email.is_none() {
                        argentum_violations.insert(
                            "email".into(),
                            Violations::new(vec!["field is required".to_string()], None),
                        );
                    }
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
                let password = raw.password;
                    if password.is_none() {
                        argentum_violations.insert(
                            "password".into(),
                            Violations::new(vec!["field is required".to_string()], None),
                        );
                    }

        if argentum_violations.is_empty() {
            Ok(Self::new(
                email.unwrap(),
                name.unwrap(),
                password.unwrap(),
            ))
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
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<UserNameRaw>,
    #[serde(rename = "password")]
    pub password: Option<String>,
}
