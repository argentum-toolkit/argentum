use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionUiWrapper {
    None,
    Block,
    Flex,
}

impl Serialize for ExtensionUiWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            ExtensionUiWrapper::None => "none",
            ExtensionUiWrapper::Block => "block",
            ExtensionUiWrapper::Flex => "flex",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for ExtensionUiWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "flex" => Ok(ExtensionUiWrapper::Flex),
            "block" => Ok(ExtensionUiWrapper::Block),
            _ => Ok(ExtensionUiWrapper::None),
        }
    }
}

impl From<ExtensionUiWrapper> for String {
    fn from(val: ExtensionUiWrapper) -> Self {
        match val {
            ExtensionUiWrapper::None => "none".to_string(),
            ExtensionUiWrapper::Block => "block".to_string(),
            ExtensionUiWrapper::Flex => "flex".to_string(),
        }
    }
}

impl From<String> for ExtensionUiWrapper {
    fn from(val: String) -> Self {
        match val.to_lowercase().as_str() {
            "block" => ExtensionUiWrapper::Block,
            "flex" => ExtensionUiWrapper::Flex,
            _ => ExtensionUiWrapper::None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_extension_ui_wrapper() {
        let none = ExtensionUiWrapper::None;
        let block = ExtensionUiWrapper::Block;
        let flex = ExtensionUiWrapper::Flex;

        assert_eq!(serde_json::to_string(&none).unwrap(), "\"none\"");
        assert_eq!(serde_json::to_string(&block).unwrap(), "\"block\"");
        assert_eq!(serde_json::to_string(&flex).unwrap(), "\"flex\"");
    }

    #[test]
    fn test_deserialize_extension_ui_wrapper() {
        let none_str = "\"none\"";
        let block_str = "\"block\"";
        let flex_str = "\"flex\"";

        let none: ExtensionUiWrapper = serde_json::from_str(none_str).unwrap();
        let block: ExtensionUiWrapper = serde_json::from_str(block_str).unwrap();
        let flex: ExtensionUiWrapper = serde_json::from_str(flex_str).unwrap();

        assert_eq!(none, ExtensionUiWrapper::None);
        assert_eq!(block, ExtensionUiWrapper::Block);
        assert_eq!(flex, ExtensionUiWrapper::Flex);
    }

    #[test]
    fn test_from_extension_ui_wrapper_to_string() {
        let none = ExtensionUiWrapper::None;
        let block = ExtensionUiWrapper::Block;
        let flex = ExtensionUiWrapper::Flex;

        assert_eq!(String::from(none), "none".to_string());
        assert_eq!(String::from(block), "block".to_string());
        assert_eq!(String::from(flex), "flex".to_string());
    }

    #[test]
    fn test_from_string_to_extension_ui_wrapper() {
        let none_str = "none".to_string();
        let block_str = "block".to_string();
        let flex_str = "flex".to_string();

        let none = ExtensionUiWrapper::from(none_str);
        let block = ExtensionUiWrapper::from(block_str);
        let flex = ExtensionUiWrapper::from(flex_str);

        assert_eq!(none, ExtensionUiWrapper::None);
        assert_eq!(block, ExtensionUiWrapper::Block);
        assert_eq!(flex, ExtensionUiWrapper::Flex);
    }
}
