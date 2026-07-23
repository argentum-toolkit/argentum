use serde::{Deserialize, Serialize};

use crate::data_type::ExtensionUi;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RefOrObject<T> {
    Ref(Reference),
    Object(T),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub reference: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "x-ag-ui")]
    pub extension_ui: Option<ExtensionUi>,
}
