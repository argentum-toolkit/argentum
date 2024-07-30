use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{License, SpecificationRoot};
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;

const PATH: &str = "/Cargo.toml";
const TEMPLATE: &str = "cargo.toml";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    package_name: String,
    description: String,
    version: String,
    license: Option<License>,
    homepage: Option<String>,
    repository: Option<String>,
    documentation: Option<String>,
}

pub(crate) struct CargoTomlGenerator {
    renderer: Arc<Renderer>,
}

impl CargoTomlGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    pub fn generate(
        &self,
        base_output_path: &str,
        spec: &SpecificationRoot,
        package_name: String,
        homepage: Option<String>,
        repository: Option<String>,
        documentation: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        let data = Data {
            package_name,
            description: spec.info.title.clone(),
            license: spec.info.license.clone(),
            version: spec.info.version.clone(),
            homepage,
            repository,
            documentation,
        };

        self.renderer
            .render(base_output_path, TEMPLATE, data, PATH)?;

        Ok(())
    }
}
