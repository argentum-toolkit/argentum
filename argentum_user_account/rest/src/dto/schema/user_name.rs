use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct UserName {
    pub first: String,

    pub last: Option<String>,

    pub patronymic: Option<String>,
}

impl UserName {
    pub fn new(first: String, last: Option<String>, patronymic: Option<String>) -> Self {
        Self {
            first,
            last,
            patronymic,
        }
    }
}

impl SerializableBody for UserName {}

impl DeserializableSchemaRaw<'_> for UserName {
    type Raw = UserNameRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let first = raw.first;
        if first.is_none() {
            argentum_violations.insert(
                "first".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let last = raw.last;
        let patronymic = raw.patronymic;

        if argentum_violations.is_empty() {
            Ok(Self::new(first.unwrap(), last, patronymic))
        } else {
            Err(Violations::new(
                vec!["wrong data for UserName".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct UserNameRaw {
    #[serde(rename = "first")]
    pub first: Option<String>,
    #[serde(rename = "last")]
    pub last: Option<String>,
    #[serde(rename = "patronymic")]
    pub patronymic: Option<String>,
}
