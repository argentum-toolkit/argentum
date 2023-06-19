use crate::description::Operation;
use crate::template::Renderer;
use convert_case::{Case, Casing};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

pub(crate) struct HandlerGenerator {
    renderer: Arc<Renderer>,
}

const MOD_PATH: &str = "/src/server/handler/mod.rs";
const MOD_TEMPLATE: &str = "server/handler.mod";
const ITEM_TEMPLATE: &str = "server/handler.item";

impl HandlerGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    fn generate_item(&self, operation: &Operation) -> Result<(), Box<dyn Error>> {
        let file_path = format!(
            "/src/server/handler/{}_handler.rs",
            operation.id.to_case(Case::Snake)
        );

        let data = HashMap::from([("operation", operation)]);

        self.renderer
            .render(ITEM_TEMPLATE, &data, file_path.as_str())?;

        Ok(())
    }

    fn generate_mod(&self, operations: &[Operation]) -> Result<(), Box<dyn Error>> {
        let data = HashMap::from([("operations", operations)]);

        self.renderer.render(MOD_TEMPLATE, &data, MOD_PATH)
    }

    pub fn generate(&self, operations: &[Operation]) -> Result<(), Box<dyn Error>> {
        self.generate_mod(operations)?;

        for operation in operations.into_iter() {
            self.generate_item(operation)?;
        }

        Ok(())
    }
}
