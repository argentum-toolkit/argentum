use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

use crate::dto::schema::Violation;
use crate::dto::schema::ViolationRaw;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct ProblemDetail {
    pub body: Option<Violation>,

    pub detail: Option<String>,

    pub status: i32,

    pub title: String,

    pub r#type: Option<String>,
}

impl ProblemDetail {
    pub fn new(
        body: Option<Violation>,
        detail: Option<String>,
        status: i32,
        title: String,
        r#type: Option<String>,
    ) -> Self {
        Self {
            body,
            detail,
            status,
            title,
            r#type,
        }
    }
}

impl SerializableBody for ProblemDetail {}

impl DeserializableSchemaRaw<'_> for ProblemDetail {
    type Raw = ProblemDetailRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let body = match Violation::try_from_raw(raw.body.unwrap()) {
            Ok(value) => Some(value),
            Err(v) => {
                argentum_violations.insert("body".into(), v);

                None
            }
        };
        let detail = raw.detail;
        let status = raw.status;
        if status.is_none() {
            argentum_violations.insert(
                "status".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let title = raw.title;
        if title.is_none() {
            argentum_violations.insert(
                "title".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let r#type = raw.r#type;

        if argentum_violations.is_empty() {
            Ok(Self::new(
                body,
                detail,
                status.unwrap(),
                title.unwrap(),
                r#type,
            ))
        } else {
            Err(Violations::new(
                vec!["wrong data for ProblemDetail".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ProblemDetailRaw {
    #[serde(rename = "body")]
    pub body: Option<ViolationRaw>,
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    #[serde(rename = "status")]
    pub status: Option<i32>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
