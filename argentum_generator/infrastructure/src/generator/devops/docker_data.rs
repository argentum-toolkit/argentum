use serde::Serialize;

/// Template data shared by all devops generators.
/// Case variants are derived in templates via handlebars helpers
/// (`kebab`, `upper_snake`).
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DevopsDocker {
    /// snake_case, e.g. `argentum`
    pub project_prefix: String,
    /// snake_case, e.g. `user_account`
    pub component_name: String,
}

impl DevopsDocker {
    pub fn new(project_prefix: String, component_name: String) -> Self {
        use convert_case::{Case, Casing};

        Self {
            project_prefix: project_prefix.to_case(Case::Snake),
            component_name: component_name.to_case(Case::Snake),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_component_name_to_snake_case() {
        let data = DevopsDocker::new("Argentum".to_string(), "UserAccount".to_string());

        assert_eq!(data.project_prefix, "argentum");
        assert_eq!(data.component_name, "user_account");
    }
}
