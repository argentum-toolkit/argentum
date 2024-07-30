use argentum_openapi_infrastructure::data_type::SchemaFormat::{Custom, Standard};
use argentum_openapi_infrastructure::data_type::{Schema, StandardFormat};

pub struct StringFactory {}

impl StringFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create(&self, schema: Schema, name: String) -> String {
        match schema.format {
            Some(Standard(StandardFormat::Uuid)) => {
                format!(
                    "(?<{}>[0-9a-f]{{8}}-[0-9a-f]{{4}}-[0-9a-f]{{4}}-[0-9a-f]{{4}}-[0-9a-f]{{12}})",
                    name
                )
            }
            Some(Standard(StandardFormat::DateTime)) => {
                format!(
                    "(?<{}>\\d{{4}}-\\d{{2}}-\\d{{2}} \\d{{2}}:\\d{{2}}:\\d{{2}}+)",
                    name
                )
            }
            Some(Custom(_custom_format)) => {
                //TODO: create regex factory for custom format
                format!("(?<{}>\\w+)", name)
            }
            _ => format!("(?<{}>\\w+)", name),
        }
    }
}
