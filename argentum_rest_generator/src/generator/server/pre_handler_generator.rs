use crate::description::Operation;
use crate::template::Renderer;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

pub(crate) struct PreHandlerGenerator {
    renderer: Arc<Renderer>,
}

const PATH: &str = "/src/server/pre_handler.rs";
const TEMPLATE: &str = "server/pre_handler";

impl PreHandlerGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    pub fn generate(&self, operations: &[Operation]) -> Result<(), Box<dyn Error>> {
        let data = HashMap::from([("operations", operations)]);

        self.renderer.render(TEMPLATE, &data, PATH)?;

        Ok(())
    }
}
