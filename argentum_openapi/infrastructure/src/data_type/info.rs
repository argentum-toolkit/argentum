use crate::data_type::{Contact, License};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: String,

    pub version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub licence: Option<License>,
}
