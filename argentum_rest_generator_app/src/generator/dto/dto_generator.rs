use crate::template::Renderer;
use std::error::Error;
use std::sync::Arc;

pub(crate) struct DtoGenerator {
    renderer: Arc<Renderer>,
}

const MOD_PATH: &str = "/src/dto/mod.rs";
const MOD_TEMPLATE: &str = "dto/mod";

impl DtoGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    pub fn generate(&self, base_output_path: &str) -> Result<(), Box<dyn Error>> {
        self.renderer
            .render(base_output_path, MOD_TEMPLATE, "", MOD_PATH)?;
        Ok(())
    }
}
