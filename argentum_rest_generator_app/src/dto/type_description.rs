use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TypeDescription {
    pub data_type: String,
    pub raw_type: String,
    pub default_initializer: String,
    pub is_ref: bool,
}

impl TypeDescription {
    pub fn new(
        data_type: impl Into<String>,
        raw_type: impl Into<String>,
        default_initializer: impl Into<String>,
        is_ref: bool,
    ) -> Self {
        Self {
            data_type: data_type.into(),
            raw_type: raw_type.into(),
            default_initializer: default_initializer.into(),
            is_ref,
        }
    }
}
