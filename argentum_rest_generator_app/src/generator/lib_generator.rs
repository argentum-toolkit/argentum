use crate::template::Renderer;

use std::collections::HashMap;
use std::sync::Arc;

pub(crate) struct LibGenerator {
    renderer: Arc<Renderer>,
}

const PATH: &str = "/src/lib.rs";
const TEMPLATE: &str = "lib";

impl LibGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    pub fn generate(&self, base_output_path: &str, package_name: &str) -> Result<(), String> {
        let data = HashMap::from([("packageName", package_name.to_owned())]);

        self.renderer
            .render(base_output_path, TEMPLATE, data, PATH)?;

        Ok(())
    }
}
