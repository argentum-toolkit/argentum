use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::generator::FileGenerator;

const README_TEMPLATE: &str = "api_app/readme.adoc";

pub struct ApiAppReadmeGenerator<L>
where
    L: LoggerTrait,
{
    file_generator: Arc<FileGenerator<L>>,
}

impl<L> ApiAppReadmeGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file_generator: Arc<FileGenerator<L>>) -> Self {
        Self { file_generator }
    }

    pub fn generate(&self, project_prefix: &str, component_name: &str) -> Result<(), String> {
        let output_path = format!("{project_prefix}_{component_name}/api-app/readme.adoc");

        self.file_generator.generate(&output_path, README_TEMPLATE)
    }
}
