use serde::Serialize;

/// Template data shared by all api-doc generators.
/// Case variants are derived in templates via handlebars helpers
/// (`kebab`, `upper_camel`, `title`).
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiDocData {
    /// snake_case, e.g. `user_account`
    pub component_name: String,
}

impl ApiDocData {
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
        let data = ApiDocData::new("UserAccount".to_string());

        assert_eq!(data.component_name, "user_account");
    }
}
