use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::generator::FileGenerator;

const MAIN_TEMPLATE: &str = "api_app/src/migration/main";

pub struct ApiAppMigrationMainGenerator<L>
where
    L: LoggerTrait,
{
    file_generator: Arc<FileGenerator<L>>,
}

impl<L> ApiAppMigrationMainGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file_generator: Arc<FileGenerator<L>>) -> Self {
        Self { file_generator }
    }

    pub fn generate(&self, project_prefix: &str, component_name: &str) -> Result<(), String> {
        let output_path =
            format!("{project_prefix}_{component_name}/api-app/src/migration/main.rs");

        self.file_generator.generate(&output_path, MAIN_TEMPLATE)
    }
}
