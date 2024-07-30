use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{License, SpecificationRoot};
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;

const PATH: &str = "/readme.adoc";
const TEMPLATE: &str = "readme.adoc";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    package_name: String,
    title: String,
    description: Option<String>,
    version: String,
    license: Option<License>,
    homepage: Option<String>,
    repository: Option<String>,
    documentation: Option<String>,
}

pub(crate) struct ReadmeAdocGenerator {
    renderer: Arc<Renderer>,
}

impl ReadmeAdocGenerator {
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
            title: spec.info.title.clone(),
            description: spec.info.description.clone(),
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
