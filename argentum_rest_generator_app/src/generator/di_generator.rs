use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::SpecificationRoot;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

pub(crate) struct DiGenerator {
    renderer: Arc<Renderer>,
}

const PATH: &str = "/src/di.rs";
const TEMPLATE: &str = "di";

impl DiGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    pub fn generate(&self, spec: &SpecificationRoot) -> Result<(), Box<dyn Error>> {
        let operations = spec.operations();
        let data = HashMap::from([("operations", operations)]);

        self.renderer.render(TEMPLATE, data, PATH)?;

        Ok(())
    }
}
