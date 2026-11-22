use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::cli_params::ApiDocParams;
use crate::generator::FileExistenceChecker;
use crate::generator::api_doc::api_doc_data::ApiDocData;

const TEMPLATE: &str = "api_doc/type.oas.yaml";

pub(crate) struct TypeGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> TypeGenerator<L>
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

        let output_path = format!("{}/api-doc/type.oas.yaml", data.component_name);

        self.file_existence_checker.ensure_absent(&output_path)?;

        self.renderer.render("", TEMPLATE, &data, &output_path)?;

        self.logger.info("type.oas.yaml is generated.");

        Ok(())
    }
}
