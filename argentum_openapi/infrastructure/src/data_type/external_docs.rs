use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocs {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    url: String,
}
