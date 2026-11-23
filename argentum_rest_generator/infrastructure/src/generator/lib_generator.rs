use crate::template::Renderer;

use std::sync::Arc;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    component_name: String,
}

pub(crate) struct LibGenerator {
    renderer: Arc<Renderer>,
}

const PATH: &str = "/src/lib.rs";
const TEMPLATE: &str = "lib";

impl LibGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    pub fn generate(&self, base_output_path: &str, component_name: &str) -> Result<(), String> {
        let data = Data {
            component_name: component_name.to_owned(),
        };

        self.renderer
            .render(base_output_path, TEMPLATE, data, PATH)?;

        Ok(())
    }
}
