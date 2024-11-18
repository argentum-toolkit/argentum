use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationItem, ViolationObject, Violations,
};
use std::collections::BTreeMap;

use crate::dto::schema::ViolationErrors;
use crate::dto::schema::ViolationErrorsRaw;
use crate::dto::schema::ViolationItems;
use crate::dto::schema::ViolationItemsRaw;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Violation {
    pub errors: Option<ViolationErrors>,

    pub items: Option<ViolationItems>,
}

impl Violation {
    pub fn new(errors: Option<ViolationErrors>, items: Option<ViolationItems>) -> Self {
        Self { errors, items }
    }
}

impl SerializableBody for Violation {}

impl DeserializableSchemaRaw<'_> for Violation {
    type Raw = ViolationRaw;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationObject = BTreeMap::new();

        let errors = match ViolationErrors::try_from_raw(raw.errors.unwrap()) {
            Ok(value) => Some(value),
            Err(v) => {
                argentum_violations.insert("errors".into(), v);

                None
            }
        };
        let items = match ViolationItems::try_from_raw(raw.items.unwrap()) {
            Ok(value) => Some(value),
            Err(v) => {
                argentum_violations.insert("items".into(), v);

                None
            }
        };

        if argentum_violations.is_empty() {
            Ok(Self::new(errors, items))
        } else {
            Err(Violations::new(
                vec!["wrong data for Violation".to_string()],
                Some(ViolationItem::Object(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ViolationRaw {
    #[serde(rename = "errors")]
    pub errors: Option<ViolationErrorsRaw>,
    #[serde(rename = "items")]
    pub items: Option<ViolationItemsRaw>,
}
