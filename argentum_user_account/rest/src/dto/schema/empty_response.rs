use argentum_rest_infrastructure::data_type::DeserializableSchemaRaw;
use argentum_rest_infrastructure::data_type::SerializableBody;
use argentum_standard_business::invariant_violation::InvariantResult;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct EmptyResponse {}

impl EmptyResponse {
    pub fn new() -> Self {
        Self {}
    }
}

impl SerializableBody for EmptyResponse {}

impl DeserializableSchemaRaw<'_> for EmptyResponse {
    type Raw = EmptyResponseRaw;

    fn try_from_raw(_: Self::Raw) -> InvariantResult<Self> {
        Ok(Self::new())
    }
}

#[derive(serde::Deserialize)]
pub struct EmptyResponseRaw {}
