use std::{error::Error, sync::Arc};

use argentum_openapi_infrastructure::data_type::SpecificationRoot;

use crate::template::Renderer;

use super::FormDataGenerator;

const MOD_PATH: &str = "/src/ui/mod.rs";
const MOD_TEMPLATE: &str = "ui/mod";

pub struct UiGenerator {
    renderer: Arc<Renderer>,
    form_data_generator: Arc<FormDataGenerator>,
}

impl UiGenerator {
    pub fn new(renderer: Arc<Renderer>, form_data_generator: Arc<FormDataGenerator>) -> Self {
        Self {
            renderer,
            form_data_generator,
        }
    }

    pub fn generate(
        &self,
        base_output_path: &str,
        spec: &SpecificationRoot,
    ) -> Result<(), Box<dyn Error>> {
        self.form_data_generator.generate(base_output_path, spec)?;

        self.renderer
            .render(base_output_path, MOD_TEMPLATE, "", MOD_PATH)?;

        Ok(())
    }
}
