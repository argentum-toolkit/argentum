use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::{
    InvariantResult, ViolationArray, ViolationItem, Violations,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct ViolationErrors(Vec<String>);

impl ViolationErrors {
    pub fn new(items: Vec<String>) -> Self {
        Self(items)
    }
}

impl SerializableBody for ViolationErrors {}

impl DeserializableSchemaRaw<'_> for ViolationErrors {
    type Raw = ViolationErrorsRaw;

    fn try_from_raw(raw_list: Self::Raw) -> InvariantResult<Self> {
        let mut argentum_violations: ViolationArray = vec![];

        let mut items = vec![];
        for raw in raw_list.0 {
            match raw {
                Some(r) => match String::try_from_raw(r) {
                    Ok(item) => {
                        items.push(item);
                    }
                    Err(e) => {
                        argentum_violations.push(e);
                    }
                },
                None => argentum_violations.push(Violations::new(
                    vec!["Array item should not be None".to_string()],
                    None,
                )),
            };
        }

        if argentum_violations.is_empty() {
            Ok(Self::new(items))
        } else {
            Err(Violations::new(
                vec!["wrong data for ViolationErrors".to_string()],
                Some(ViolationItem::Array(argentum_violations)),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ViolationErrorsRaw(Vec<Option<String>>);
