use crate::template::Renderer;
use std::error::Error;
use std::sync::Arc;

const MOD_PATH: &str = "/src/server/mod.rs";
const MOD_TEMPLATE: &str = "server/mod";

pub(crate) struct ServerGenerator {
    renderer: Arc<Renderer>,
}

impl ServerGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    pub fn generate(&self, base_output_path: &str) -> Result<(), Box<dyn Error>> {
        self.renderer
            .render(base_output_path, MOD_TEMPLATE, "", MOD_PATH)?;

        Ok(())
    }
}
