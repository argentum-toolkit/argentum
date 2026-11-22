use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::cli_params::InfrastructureParams;
use crate::generator::FileExistenceChecker;

use convert_case::{Case, Casing};
use serde::Serialize;

const DB_MOD_TEMPLATE: &str = "infrastructure/src/db/mod";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    component_name: String,
}

pub(crate) struct InfDbModGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> InfDbModGenerator<L>
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

    pub fn generate(&self, params: &InfrastructureParams) -> Result<(), String> {
        let component_name = params.component_name.to_case(Case::Snake);

        let output_path = format!("{}/infrastructure/src/db/mod.rs", component_name);

        let data = Data {
            component_name: component_name.clone(),
        };

        self.file_existence_checker.ensure_absent(&output_path)?;

        self.renderer
            .render("", DB_MOD_TEMPLATE, &data, &output_path)?;

        self.logger.info("DB mod is generated.");

        Ok(())
    }
}
