use crate::data_type::Schema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RefOrObject<T> {
    Ref(Reference),
    Object(T),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Obj {
    #[serde(rename = "type")]
    pub obj_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaTypeObject {
    pub schema: RefOrObject<Schema>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub reference: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestBody {
    //TODO: load body_schema:
    // pub required: Option<bool>,
    pub content: BTreeMap<String, MediaTypeObject>,
}
