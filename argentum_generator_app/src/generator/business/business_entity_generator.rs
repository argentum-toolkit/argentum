use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::cli_params::BusinessParams;
use crate::generator::FileExistenceChecker;
use crate::generator::mod_file_writer::append_mod;

use convert_case::{Case, Casing};
use serde::Serialize;

const ENTITY_TEMPLATE: &str = "business/src/entity";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    entity_name: String,
}

pub(crate) struct BusinessEntityGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> BusinessEntityGenerator<L>
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
        let entity_snake = entity_name.to_case(Case::Snake);

        let output_path = format!("{}/business/src/entity/{}.rs", component_name, entity_snake);

        let data = Data {
            entity_name: entity_name.clone(),
        };

        self.file_existence_checker.ensure_absent(&output_path)?;

        self.renderer
            .render("", ENTITY_TEMPLATE, &data, &output_path)?;

        let lines = vec![
            format!("mod {};", entity_snake),
            format!("pub use {}::{};", entity_snake, entity_name),
        ];

        let mod_path = format!("{}/business/src/entity/mod.rs", component_name);

        append_mod(&mod_path, lines)?;

        self.logger.info("Entity is generated.");

        Ok(())
    }
}
