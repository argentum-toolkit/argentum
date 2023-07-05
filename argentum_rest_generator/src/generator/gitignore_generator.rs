use crate::template::Renderer;
use std::error::Error;
use std::sync::Arc;

pub(crate) struct GitIgnoreGenerator {
    renderer: Arc<Renderer>,
}

const PATH: &str = "/.gitignore";
const TEMPLATE: &str = ".gitignore";

impl GitIgnoreGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    pub fn generate(&self) -> Result<(), Box<dyn Error>> {
        let data = "";

        self.renderer.render(TEMPLATE, data, PATH)?;

        Ok(())
    }
}
