use std::io::Write;
use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::cli_params::DbInfrastructureParams;
use convert_case::{Case, Casing};
use serde::Serialize;

const REPO_TEMPLATE: &str = "infrastructure/db_repository";
const DTO_TEMPLATE: &str = "infrastructure/db_dto";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    component_name: String,
    entity_name: String,
}

pub(crate) struct DbInfrastructureGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
}

impl<L> DbInfrastructureGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(logger: Arc<L>, renderer: Arc<Renderer>) -> Self {
        Self { logger, renderer }
    }

    fn append_mod(&self, mod_path: &str, lines: Vec<String>) -> Result<(), String> {
        if !std::path::Path::new(mod_path).exists() {
            std::fs::File::create(mod_path).map_err(|e| e.to_string())?;
        }

        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(mod_path)
            .map_err(|e| e.to_string())?;
        let content = std::fs::read_to_string(mod_path).map_err(|e| e.to_string())?;

        for line in lines {
            if !&content.contains(&line) {
                writeln!(file, "{}", line).map_err(|e| e.to_string())?
            }
        }

        Ok(())
    }

    fn generate_repository(&self, params: DbInfrastructureParams) -> Result<(), String> {
        let output_path = format!(
            "infrastructure/src/db/repository/{}_repository.rs",
            params.entity_name.to_case(Case::Snake)
        );

        let data = Data {
            component_name: params.component_name,
            entity_name: params.entity_name.clone(),
        };

        self.renderer
            .render("", REPO_TEMPLATE, &data, &output_path)?;

        let lines = vec![
            format!(
                "mod {}_repository;",
                params.entity_name.to_case(Case::Snake)
            ),
            format!(
                "pub use {}_repository::{}Repository;",
                params.entity_name.to_case(Case::Snake),
                params.entity_name
            ),
        ];

        let mod_path = "infrastructure/src/db/repository/mod.rs";

        self.append_mod(mod_path, lines)
    }

    pub fn generate_dto(&self, params: DbInfrastructureParams) -> Result<(), String> {
        let output_path = format!(
            "infrastructure/src/db/dto/{}_dto.rs",
            params.entity_name.to_case(Case::Snake)
        );

        let data = Data {
            component_name: params.component_name,
            entity_name: params.entity_name.clone(),
        };

        self.renderer
            .render("", DTO_TEMPLATE, &data, &output_path)?;

        let lines = vec![
            format!("mod {}_dto;", params.entity_name.to_case(Case::Snake)),
            format!(
                "pub(crate) use {}_dto::{}Dto;",
                params.entity_name.to_case(Case::Snake),
                params.entity_name
            ),
        ];

        let mod_path = "infrastructure/src/db/dto/mod.rs";

        self.append_mod(mod_path, lines)
    }

    pub fn generate(&self, params: DbInfrastructureParams) -> Result<(), String> {
        self.generate_dto(params.clone())?;
        self.logger.info("DTO generated.");
        self.generate_repository(params)?;
        self.logger.info("Repository generated.");

        Ok(())
    }
}
