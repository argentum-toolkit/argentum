use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiAppData {
    pub project_prefix: String,
    pub component_name: String,
    pub argentum_path: String,
}

impl ApiAppData {
    pub fn new(project_prefix: String, component_name: String, argentum_path: String) -> Self {
        use convert_case::{Case, Casing};

        Self {
            project_prefix: project_prefix.to_case(Case::Snake),
            component_name: component_name.to_case(Case::Snake),
            argentum_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_names_to_snake_case() {
        let data = ApiAppData::new(
            "Argentum".to_string(),
            "UserAccount".to_string(),
            "../..".to_string(),
        );

        assert_eq!(data.project_prefix, "argentum");
        assert_eq!(data.component_name, "user_account");
        assert_eq!(data.argentum_path, "../..");
    }
}
