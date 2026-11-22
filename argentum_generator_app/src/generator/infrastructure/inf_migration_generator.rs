use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::cli_params::InfrastructureParams;
use crate::generator::FileExistenceChecker;

use convert_case::{Case, Casing};
use serde::Serialize;

const MIGRATION_MOD_TEMPLATE: &str = "infrastructure/src/db/migration/mod";
const MIGRATION_TEMPLATE: &str = "infrastructure/src/db/migration/migration_000_init";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    component_name: String,
    entity_name: String,
}

pub(crate) struct InfMigrationGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> InfMigrationGenerator<L>
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
        let entity_name = params.entity_name.to_case(Case::Pascal);

        let migration_mod_path =
            format!("{}/infrastructure/src/db/migration/mod.rs", component_name);
        let migration_path = format!(
            "{}/infrastructure/src/db/migration/migration_000_init.rs",
            component_name
        );

        let data = Data {
            component_name: component_name.clone(),
            entity_name,
        };

        self.file_existence_checker
            .ensure_absent(&migration_mod_path)?;
        self.file_existence_checker.ensure_absent(&migration_path)?;

        self.renderer
            .render("", MIGRATION_MOD_TEMPLATE, &data, &migration_mod_path)?;

        self.renderer
            .render("", MIGRATION_TEMPLATE, &data, &migration_path)?;

        self.logger.info("Migration is generated.");

        Ok(())
    }
}
