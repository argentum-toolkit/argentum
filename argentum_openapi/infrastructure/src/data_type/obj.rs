use crate::data_type::{ExtensionUi, RefOrObject, Schema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Obj {
    #[serde(rename = "type")]
    pub obj_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaTypeObject {
    pub schema: RefOrObject<Schema>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "x-ag-ui")]
    pub extension_ui: Option<ExtensionUi>,
}
