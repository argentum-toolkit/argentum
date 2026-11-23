use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GuestSendsMessageSchema {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "captcha")]
    pub captcha: Option<String>,
    #[serde(rename = "message")]
    pub message: String,
}

impl GuestSendsMessageSchema {
    pub fn new(
        name: String,
        email: String,
        subject: String,
        captcha: Option<String>,
        message: String,
    ) -> Self {
        Self {
            name,
            email,
            subject,
            captcha,
            message,
        }
    }
}

impl SerializableBody for GuestSendsMessageSchema {}

impl DeserializableSchemaRaw<'_> for GuestSendsMessageSchema {
    type Raw = GuestSendsMessageSchemaRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let name = raw.name;
        if name.is_none() {
            argentum_violations.insert(
                "name".into(),
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
        let subject = raw.subject;
        if subject.is_none() {
            argentum_violations.insert(
                "subject".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let captcha = raw.captcha;
        let message = raw.message;
        if message.is_none() {
            argentum_violations.insert(
                "message".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }

        if argentum_violations.is_empty()
            && let Some(name) = name
            && let Some(email) = email
            && let Some(subject) = subject
            && let Some(message) = message
        {
            Ok(Self::new(name, email, subject, captcha, message))
        } else {
            Err(Violations::new(
                vec!["wrong data for GuestSendsMessageSchema".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct GuestSendsMessageSchemaRaw {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "subject")]
    pub subject: Option<String>,
    #[serde(rename = "captcha")]
    pub captcha: Option<String>,
    #[serde(rename = "message")]
    pub message: Option<String>,
}
