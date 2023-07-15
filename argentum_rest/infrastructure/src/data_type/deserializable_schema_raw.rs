use argentum_standard_business::invariant_violation::InvariantResult;
use serde::Deserialize;

pub trait DeserializableSchemaRaw<'a>: Sized {
    type Raw: Deserialize<'a>;

    fn try_from_raw(raw: Self::Raw) -> InvariantResult<Self>;
}
