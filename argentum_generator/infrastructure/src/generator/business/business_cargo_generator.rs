use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::generator::FileExistenceChecker;
use crate::params::BusinessParams;

use convert_case::{Case, Casing};
use serde::Serialize;

const CARGO_TEMPLATE: &str = "business/Cargo.toml";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    project_prefix: String,
    component_name: String,
}

pub struct BusinessCargoGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> BusinessCargoGenerator<L>
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
        let project_prefix = params.project_prefix.to_case(Case::Snake);
        let component_name = params.component_name.to_case(Case::Snake);
        let dir = format!("{}_{}", project_prefix, component_name);

        let output_path = format!("{}/business/Cargo.toml", dir);

        let data = Data {
            project_prefix: project_prefix.clone(),
            component_name: component_name.clone(),
        };

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
