use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::cli_params::ApiDocParams;
use crate::generator::FileExistenceChecker;
use crate::generator::api_doc::api_doc_data::ApiDocData;

const TEMPLATE: &str = "api_doc/paths.oas.yaml";

pub(crate) struct PathsGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> PathsGenerator<L>
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

    pub fn generate(&self, params: &ApiDocParams) -> Result<(), String> {
        let data = ApiDocData::new(params.component_name.clone());

        let output_path = format!("{}/api-doc/paths.oas.yaml", data.component_name);

        self.file_existence_checker.ensure_absent(&output_path)?;

        self.renderer.render("", TEMPLATE, &data, &output_path)?;

        self.logger.info("paths.oas.yaml is generated.");

        Ok(())
    }
}
