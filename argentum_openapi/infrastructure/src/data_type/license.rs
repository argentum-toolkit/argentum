use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct License {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}
