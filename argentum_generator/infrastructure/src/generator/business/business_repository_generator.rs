use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::generator::FileExistenceChecker;
use crate::generator::mod_file_writer::append_mod;
use crate::params::BusinessParams;

use convert_case::{Case, Casing};
use serde::Serialize;

const REPOSITORY_TEMPLATE: &str = "business/src/repository";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    entity_name: String,
}

pub struct BusinessRepositoryGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> BusinessRepositoryGenerator<L>
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
        let entity_name = params.entity_name.to_case(Case::Pascal);
        let entity_snake = entity_name.to_case(Case::Snake);

        let output_path = format!(
            "{}/business/src/repository/{}_repository.rs",
            dir, entity_snake
        );

        let data = Data {
            entity_name: entity_name.clone(),
        };

        if self.file_existence_checker.exists(&output_path) {
            self.logger
                .info(format!("{output_path} already exists, skipped."));

            return Ok(());
        }

        self.renderer
            .render("", REPOSITORY_TEMPLATE, &data, &output_path)?;

        let lines = vec![format!("pub mod {}_repository;", entity_snake)];

        let mod_path = format!("{}/business/src/repository/mod.rs", dir);

        append_mod(&mod_path, lines)?;

        self.logger.info("Repository is generated.");

        Ok(())
    }
}
