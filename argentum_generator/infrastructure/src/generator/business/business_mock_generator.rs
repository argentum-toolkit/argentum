use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::generator::FileExistenceChecker;
use crate::generator::mod_file_writer::append_mod;
use crate::params::BusinessParams;

use convert_case::{Case, Casing};
use serde::Serialize;

const MOCK_TEMPLATE: &str = "business/src/mock_repository";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    entity_name: String,
}

pub struct BusinessMockGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> BusinessMockGenerator<L>
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

        let output_path = format!(
            "{}/business/src/mock/repository/{}_repository_mock.rs",
            component_name, entity_snake
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
            .render("", MOCK_TEMPLATE, &data, &output_path)?;

        append_mod(
            &format!("{}/business/src/mock/repository/mod.rs", component_name),
            vec![format!("pub mod {}_repository_mock;", entity_snake)],
        )?;

        append_mod(
            &format!("{}/business/src/mock/mod.rs", component_name),
            vec!["pub mod repository;".to_string()],
        )?;

        for module in ["use_case", "service", "specification", "event"] {
            append_mod(
                &format!("{}/business/src/{}/mod.rs", component_name, module),
                Vec::new(),
            )?;
        }

        self.logger.info("Mock is generated.");

        Ok(())
    }
}
