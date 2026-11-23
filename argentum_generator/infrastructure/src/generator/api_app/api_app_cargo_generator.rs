use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::generator::FileExistenceChecker;
use crate::generator::api_app::api_app_data::ApiAppData;
use crate::params::ApiAppParams;

const CARGO_TEMPLATE: &str = "api_app/Cargo.toml";

pub struct ApiAppCargoGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> ApiAppCargoGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(
        logger: Arc<L>,
        renderer: Arc<Renderer>,
        file_existence_checker: Arc<FileExistenceChecker>,
    ) -> Self {
        Self {
            logger,
            renderer,
            file_existence_checker,
        }
    }

    pub fn generate(&self, params: &ApiAppParams, argentum_path: &str) -> Result<(), String> {
        let data = ApiAppData::new(
            params.project_prefix.clone(),
            params.component_name.clone(),
            argentum_path.to_string(),
        );

        let output_path = format!(
            "{}_{}/api-app/Cargo.toml",
            data.project_prefix, data.component_name
        );

        if self.file_existence_checker.exists(&output_path) {
            self.logger
                .info(format!("{output_path} already exists, skipped."));

            return Ok(());
        }

        self.renderer
            .render("", CARGO_TEMPLATE, &data, &output_path)?;

        self.logger.info("Cargo.toml is generated.");

        Ok(())
    }
}
