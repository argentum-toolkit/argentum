use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::generator::FileExistenceChecker;
use crate::params::BusinessParams;

use convert_case::{Case, Casing};
use serde::Serialize;

const DI_TEMPLATE: &str = "business/src/di";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    component_name: String,
    entity_name: String,
}

pub struct BusinessDiGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> BusinessDiGenerator<L>
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

    pub fn generate(&self, params: &BusinessParams) -> Result<(), String> {
        let component_name = params.component_name.to_case(Case::Snake);
        let entity_name = params.entity_name.to_case(Case::Pascal);

        let output_path = format!("{}/business/src/di.rs", component_name);

        let data = Data {
            component_name: component_name.clone(),
            entity_name,
        };

        self.file_existence_checker.ensure_absent(&output_path)?;

        self.renderer.render("", DI_TEMPLATE, &data, &output_path)?;

        self.logger.info("DI is generated.");

        Ok(())
    }
}
