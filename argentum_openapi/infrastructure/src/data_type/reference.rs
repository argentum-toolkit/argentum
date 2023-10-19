use serde::{Deserialize, Serialize};

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
}

