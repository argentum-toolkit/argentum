use crate::data_type::RefOrObject;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, RefOrObject<Self>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,

    #[serde(rename = "type")]
    pub schema_type: Option<SchemaType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SchemaFormat>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Scalar {
    pub format: Option<SchemaFormat>,

    #[serde(rename = "type")]
    pub schema_type: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    String,
    Integer,
    Number,
    Boolean,
    Array,
    Object,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase", untagged)]
pub enum SchemaFormat {
    /// Use to define additional detail about the value.
    Standard(StandardFormat),
    /// Can be used to provide additional detail about the value when [`SchemaFormat::KnownFormat`]
    /// is not suitable.
    Custom(String),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum StandardFormat {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float,
    Double,
    /// base64 encoded chars.
    Byte,
    /// binary data (octet).
    Binary,
    /// ISO-8601 full date [FRC3339](https://xml2rfc.ietf.org/public/rfc/html/rfc3339.html#anchor14).
    Date,
    /// ISO-8601 full date time [FRC3339](https://xml2rfc.ietf.org/public/rfc/html/rfc3339.html#anchor14).
    #[serde(rename = "date-time")]
    DateTime,
    /// Hint to UI to obscure input.
    Password,
    Uuid,
}
