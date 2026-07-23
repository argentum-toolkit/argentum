use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionUiOptionalStyle {
    None,
    Normal,
    Inverted,
}

impl Serialize for ExtensionUiOptionalStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            ExtensionUiOptionalStyle::None => "none",
            ExtensionUiOptionalStyle::Normal => "normal",
            ExtensionUiOptionalStyle::Inverted => "inverted",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for ExtensionUiOptionalStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "inverted" => Ok(ExtensionUiOptionalStyle::Inverted),
            "normal" => Ok(ExtensionUiOptionalStyle::Normal),
            _ => Ok(ExtensionUiOptionalStyle::None),
        }
    }
}

impl From<ExtensionUiOptionalStyle> for String {
    fn from(val: ExtensionUiOptionalStyle) -> Self {
        match val {
            ExtensionUiOptionalStyle::None => "none".to_string(),
            ExtensionUiOptionalStyle::Normal => "normal".to_string(),
            ExtensionUiOptionalStyle::Inverted => "inverted".to_string(),
        }
    }
}

impl From<String> for ExtensionUiOptionalStyle {
    fn from(val: String) -> Self {
        match val.to_lowercase().as_str() {
            "normal" => ExtensionUiOptionalStyle::Normal,
            "inverted" => ExtensionUiOptionalStyle::Inverted,
            _ => ExtensionUiOptionalStyle::None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_extension_ui_wrapper() {
        let none = ExtensionUiOptionalStyle::None;
        let normal = ExtensionUiOptionalStyle::Normal;
        let inverted = ExtensionUiOptionalStyle::Inverted;

        assert_eq!(serde_json::to_string(&none).unwrap(), "\"none\"");
        assert_eq!(serde_json::to_string(&normal).unwrap(), "\"normal\"");
        assert_eq!(serde_json::to_string(&inverted).unwrap(), "\"inverted\"");
    }

    #[test]
    fn test_deserialize_extension_ui_wrapper() {
        let none_str = "\"none\"";
        let normal_str = "\"normal\"";
        let inverted_str = "\"inverted\"";

        let none: ExtensionUiOptionalStyle = serde_json::from_str(none_str).unwrap();
        let normal: ExtensionUiOptionalStyle = serde_json::from_str(normal_str).unwrap();
        let inverted: ExtensionUiOptionalStyle = serde_json::from_str(inverted_str).unwrap();

        assert_eq!(none, ExtensionUiOptionalStyle::None);
        assert_eq!(normal, ExtensionUiOptionalStyle::Normal);
        assert_eq!(inverted, ExtensionUiOptionalStyle::Inverted);
    }

    #[test]
    fn test_from_extension_ui_wrapper_to_string() {
        let none = ExtensionUiOptionalStyle::None;
        let normal = ExtensionUiOptionalStyle::Normal;
        let inverted = ExtensionUiOptionalStyle::Inverted;

        assert_eq!(String::from(none), "none".to_string());
        assert_eq!(String::from(normal), "normal".to_string());
        assert_eq!(String::from(inverted), "inverted".to_string());
    }

    #[test]
    fn test_from_string_to_extension_ui_wrapper() {
        let none_str = "none".to_string();
        let normal_str = "normal".to_string();
        let inverted_str = "inverted".to_string();

        let none = ExtensionUiOptionalStyle::from(none_str);
        let normal = ExtensionUiOptionalStyle::from(normal_str);
        let inverted = ExtensionUiOptionalStyle::from(inverted_str);

        assert_eq!(none, ExtensionUiOptionalStyle::None);
        assert_eq!(normal, ExtensionUiOptionalStyle::Normal);
        assert_eq!(inverted, ExtensionUiOptionalStyle::Inverted);
    }
}
