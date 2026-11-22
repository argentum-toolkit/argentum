use serde::Serialize;

/// Template data shared by all ui generators.
/// Case variants are derived in templates via handlebars helpers
/// (`upper_camel`, `title`).
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UiData {
    /// snake_case, e.g. `user_account`
    pub component_name: String,
}

impl UiData {
    pub fn new(component_name: String) -> Self {
        use convert_case::{Case, Casing};

        Self {
            component_name: component_name.to_case(Case::Snake),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_component_name_to_snake_case() {
        let data = UiData::new("UserAccount".to_string());

        assert_eq!(data.component_name, "user_account");
    }
}
