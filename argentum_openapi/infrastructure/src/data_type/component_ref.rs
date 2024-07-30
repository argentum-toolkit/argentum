use crate::data_type::ComponentType;

pub struct ComponentRef {
    pub file_path: Option<String>,
    component_type: ComponentType,
    pub component_name: String,
}

impl ComponentRef {
    pub fn is_schema(&self) -> bool {
        self.component_type == ComponentType::Schema
    }

    pub fn is_request_body(&self) -> bool {
        self.component_type == ComponentType::RequestBody
    }

    pub fn is_response(&self) -> bool {
        self.component_type == ComponentType::Response
    }
}

impl From<String> for ComponentRef {
    fn from(value: String) -> Self {
        let parts = value.split("#/").collect::<Vec<_>>();

        if parts.clone().len() != 2 {
            panic!("Wrong format of reference {}", value)
        }

        let file_path = if parts[0].is_empty() {
            None
        } else {
            Some(parts[0].to_string())
        };

        let component_path = parts
            .last()
            .unwrap_or_else(|| panic!("Wrong component path of reference {}", value));

        let component_parts = component_path.split('/').collect::<Vec<_>>();

        let component_type = ComponentType::from(component_parts[1]);
        if component_parts.clone().len() != 3 || component_parts[0] != "components" {
            panic!(
                "Wrong component path {}. Expected: `#/components/<type>/<{{name}}>`",
                component_path
            )
        }

        let component_name = component_parts[2].to_string();

        Self {
            file_path,
            component_type,
            component_name,
        }
    }
}
