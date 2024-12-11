use std::{error::Error, sync::Arc};

use argentum_openapi_infrastructure::data_type::SpecificationRoot;

use super::FormDataGenerator;

pub struct UiGenerator {
    form_data_generator: Arc<FormDataGenerator>,
}

impl UiGenerator {
    pub fn new(form_data_generator: Arc<FormDataGenerator>) -> Self {
        Self {
            form_data_generator,
        }
    }

    pub fn generate(
        &self,
        base_output_path: &str,
        spec: &SpecificationRoot,
    ) -> Result<(), Box<dyn Error>> {
        self.form_data_generator.generate(base_output_path, spec)?;

        Ok(())
    }
}
