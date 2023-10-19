use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;


#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct LoginWithPasswordSchema {
    pub email: String,

    pub password: String,

}

impl LoginWithPasswordSchema {
    pub fn new(
        email: String,
        password: String,
    ) -> Self {
        Self {
            email,
            password,
        }
    }
}

impl SerializableBody for LoginWithPasswordSchema {}

impl DeserializableSchemaRaw<'_> for LoginWithPasswordSchema {
    type Raw = LoginWithPasswordSchemaRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

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

        if argentum_violations.is_empty() {
            Ok(Self::new(
                email.unwrap(),
                password.unwrap(),
            ))
        } else {
            Err(Violations::new(
                vec!["wrong data for LoginWithPasswordSchema".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }

}

#[derive(serde::Deserialize)]
pub struct LoginWithPasswordSchemaRaw {
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "password")]
    pub password: Option<String>,
}
