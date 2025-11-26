use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct ExchangeCodeSchema {
    pub code: String,
    pub state: Option<String>,
}

impl ExchangeCodeSchema {
    pub fn new(code: String, state: Option<String>) -> Self {
        Self { code, state }
    }
}

impl SerializableBody for ExchangeCodeSchema {}

impl DeserializableSchemaRaw<'_> for ExchangeCodeSchema {
    type Raw = ExchangeCodeSchemaRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let code = raw.code;
        if code.is_none() {
            argentum_violations.insert(
                "code".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }
        let state = raw.state;

        if argentum_violations.is_empty()
            && let Some(code) = code
        {
            Ok(Self::new(code, state))
        } else {
            Err(Violations::new(
                vec!["wrong data for ExchangeCodeSchema".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ExchangeCodeSchemaRaw {
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[serde(rename = "state")]
    pub state: Option<String>,
}
