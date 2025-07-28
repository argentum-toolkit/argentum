use std::{error::Error, sync::Arc};

use argentum_openapi_infrastructure::data_type::SpecificationRoot;

use crate::generator::ui::FormGenerator;
use crate::generator::ui::InputGenerator;
use crate::template::Renderer;

use super::CallbacksGenerator;
use super::FormProcessorGenerator;

const MOD_PATH: &str = "/src/ui/mod.rs";
const MOD_TEMPLATE: &str = "ui/mod";

pub struct UiGenerator {
    renderer: Arc<Renderer>,
    form_generator: Arc<FormGenerator>,
    form_data_generator: Arc<CallbacksGenerator>,
    input_generator: Arc<InputGenerator>,
    web_boilerplate_generator: Arc<FormProcessorGenerator>,
}

impl UiGenerator {
    pub fn new(
        renderer: Arc<Renderer>,
        form_generator: Arc<FormGenerator>,
        form_data_generator: Arc<CallbacksGenerator>,
        input_generator: Arc<InputGenerator>,
        web_boilerplate_generator: Arc<FormProcessorGenerator>,
    ) -> Self {
        Self {
            renderer,
            form_generator,
            form_data_generator,
            input_generator,
            web_boilerplate_generator,
        }
    }

    pub fn generate(
        &self,
        base_output_path: &str,
        spec: &SpecificationRoot,
    ) -> Result<(), Box<dyn Error>> {
        self.form_generator.generate(base_output_path, spec)?;
        self.form_data_generator.generate(base_output_path, spec)?;
        self.input_generator.generate(base_output_path, spec)?;
        self.web_boilerplate_generator
            .generate(base_output_path, spec)?;

        self.renderer
            .render(base_output_path, MOD_TEMPLATE, "", MOD_PATH)?;

        Ok(())
    }
}
