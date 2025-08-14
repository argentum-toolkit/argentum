use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{Operation, SpecificationRoot};
use std::sync::Arc;

const PATH: &str = "/src/server/pre_handler.rs";
const TEMPLATE: &str = "server/pre_handler";

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    pub operations: Vec<Operation>,
    pub security_enabled: bool,
}

pub(crate) struct PreHandlerGenerator {
    renderer: Arc<Renderer>,
}

impl PreHandlerGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    pub fn generate(&self, base_output_path: &str, spec: &SpecificationRoot) -> Result<(), String> {
        let operations = spec.operations();

        let mut security_enabled = false;

        for operation in operations.clone().into_iter() {
            if operation.security.is_some() {
                security_enabled = true;

                break;
            }
        }

        let data = Data {
            operations,
            security_enabled,
        };

        self.renderer
            .render(base_output_path, TEMPLATE, data, PATH)?;

        Ok(())
    }
}
