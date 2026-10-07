use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct ExchangeCodeSchema {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "provider")]
    pub provider: String,
}

impl ExchangeCodeSchema {
    pub fn new(code: String, provider: String) -> Self {
        Self { code, provider }
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
        let provider = raw.provider;
        if provider.is_none() {
            argentum_violations.insert(
                "provider".into(),
                Violations::new(vec!["field is required".to_string()], None),
            );
        }

        if argentum_violations.is_empty()
            && let Some(code) = code
            && let Some(provider) = provider
        {
            Ok(Self::new(code, provider))
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
    #[serde(rename = "provider")]
    pub provider: Option<String>,
}
