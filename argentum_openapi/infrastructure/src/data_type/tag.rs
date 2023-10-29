use crate::data_type::ExternalDocs;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocs>,
}
