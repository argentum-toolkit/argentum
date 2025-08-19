use serde::{Deserialize, Serialize};

fn default_submit_port() -> String {
    "Submit".into()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionForm {
    #[serde(default = "default_submit_port", rename = "submit-label")]
    pub submit_label: String,
}
