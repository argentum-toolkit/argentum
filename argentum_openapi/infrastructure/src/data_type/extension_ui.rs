use serde::{Deserialize, Serialize};

use crate::data_type::{ExtensionUiOptionalStyle, ExtensionUiWrapper};

fn default_wrapper() -> ExtensionUiWrapper {
    ExtensionUiWrapper::None
}

fn default_optional_style() -> ExtensionUiOptionalStyle {
    ExtensionUiOptionalStyle::None
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionUi {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i16>,

    #[serde(default = "default_wrapper")]
    pub wrapper: ExtensionUiWrapper,

    #[serde(skip_serializing_if = "Option::is_none", rename = "optional-title")]
    pub optional_title: Option<String>,

    #[serde(rename = "optional-style", default = "default_optional_style")]
    pub optional_style: ExtensionUiOptionalStyle,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_extension_ui() {
        let extension_ui = ExtensionUi {
            id: Some("id1".to_string()),
            name: Some("name1".to_string()),
            label: Some("label1".to_string()),
            weight: Some(1),
            wrapper: ExtensionUiWrapper::Flex,
            optional_title: Some("Enable optional inputs".into()),
            optional_style: ExtensionUiOptionalStyle::Normal,
        };

        let serialized = serde_json::to_string(&extension_ui).unwrap();
        let expected = r#"{"id":"id1","name":"name1","label":"label1","weight":1,"wrapper":"flex","optional-title":"Enable optional inputs","optional-style":"normal"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialize_extension_ui() {
        let json_str = r#"{"id":"id1","name":"name1","label":"label1","weight":1,"wrapper":"flex","optional-title":"Enable optional inputs","optional-style":"normal"}"#;
        let extension_ui: ExtensionUi = serde_json::from_str(json_str).unwrap();

        let expected = ExtensionUi {
            id: Some("id1".to_string()),
            name: Some("name1".to_string()),
            label: Some("label1".to_string()),
            weight: Some(1),
            wrapper: ExtensionUiWrapper::Flex,
            optional_title: Some("Enable optional inputs".into()),
            optional_style: ExtensionUiOptionalStyle::Normal,
        };

        assert_eq!(extension_ui, expected);
    }

    #[test]
    fn test_serialize_extension_ui_with_defaults() {
        let extension_ui = ExtensionUi {
            id: None,
            name: None,
            label: None,
            weight: None,
            wrapper: ExtensionUiWrapper::None,
            optional_title: None,
            optional_style: ExtensionUiOptionalStyle::None,
        };

        let serialized = serde_json::to_string(&extension_ui).unwrap();
        let expected = r#"{"wrapper":"none","optional-style":"none"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialize_extension_ui_with_defaults() {
        let json_str = r#"{"wrapper":"none","optional-style":"none"}"#;
        let extension_ui: ExtensionUi = serde_json::from_str(json_str).unwrap();

        let expected = ExtensionUi {
            id: None,
            name: None,
            label: None,
            weight: None,
            wrapper: ExtensionUiWrapper::None,
            optional_title: None,
            optional_style: ExtensionUiOptionalStyle::None,
        };

        assert_eq!(extension_ui, expected);
    }
}
