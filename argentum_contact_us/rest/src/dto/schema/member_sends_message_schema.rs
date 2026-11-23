use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct MemberSendsMessageSchema {
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "captcha")]
    pub captcha: Option<String>,
    #[serde(rename = "message")]
    pub message: String,
}

impl MemberSendsMessageSchema {
    pub fn new(subject: String, captcha: Option<String>, message: String) -> Self {
        Self {
            subject,
            captcha,
            message,
        }
    }
}

impl SerializableBody for MemberSendsMessageSchema {}

impl DeserializableSchemaRaw<'_> for MemberSendsMessageSchema {
    type Raw = MemberSendsMessageSchemaRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

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
            && let Some(subject) = subject
            && let Some(message) = message
        {
            Ok(Self::new(subject, captcha, message))
        } else {
            Err(Violations::new(
                vec!["wrong data for MemberSendsMessageSchema".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct MemberSendsMessageSchemaRaw {
    #[serde(rename = "subject")]
    pub subject: Option<String>,
    #[serde(rename = "captcha")]
    pub captcha: Option<String>,
    #[serde(rename = "message")]
    pub message: Option<String>,
}
