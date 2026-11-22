use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::generator::FileExistenceChecker;
use crate::generator::mod_file_writer::append_mod;
use crate::params::DbInfrastructureParams;
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

pub struct DbInfrastructureGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> DbInfrastructureGenerator<L>
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

    fn generate_repository(&self, params: DbInfrastructureParams) -> Result<(), String> {
        let component_name = params.component_name;
        let entity_name = params.entity_name;

        let output_path = format!(
            "{}/infrastructure/src/db/repository/{}_repository.rs",
            component_name.to_case(Case::Snake),
            entity_name.to_case(Case::Snake)
        );

        let data = Data {
            component_name: component_name.clone(),
            entity_name: entity_name.clone(),
        };

        if self.file_existence_checker.exists(&output_path) {
            self.logger
                .info(format!("{output_path} already exists, skipped."));

            return Ok(());
        }

        self.renderer
            .render("", REPO_TEMPLATE, &data, &output_path)?;

        let lines = vec![
            format!("mod {}_repository;", entity_name.to_case(Case::Snake)),
            format!(
                "pub use {}_repository::{}Repository;",
                entity_name.to_case(Case::Snake),
                entity_name
            ),
        ];

        let mod_path = format!(
            "{}/infrastructure/src/db/repository/mod.rs",
            component_name.to_case(Case::Snake)
        );

        append_mod(&mod_path, lines)
    }

    pub fn generate_dto(&self, params: DbInfrastructureParams) -> Result<(), String> {
        let component_name = params.component_name;
        let entity_name = params.entity_name;

        let output_path = format!(
            "{}/infrastructure/src/db/dto/{}_dto.rs",
            component_name.to_case(Case::Snake),
            entity_name.to_case(Case::Snake),
        );

        let data = Data {
            component_name: component_name.clone(),
            entity_name: entity_name.clone(),
        };

        if self.file_existence_checker.exists(&output_path) {
            self.logger
                .info(format!("{output_path} already exists, skipped."));

            return Ok(());
        }

        self.renderer
            .render("", DTO_TEMPLATE, &data, &output_path)?;

        let lines = vec![
            format!("mod {}_dto;", entity_name.to_case(Case::Snake)),
            format!(
                "pub(crate) use {}_dto::{}Dto;",
                entity_name.to_case(Case::Snake),
                entity_name
            ),
        ];

        let mod_path = format!(
            "{}/infrastructure/src/db/dto/mod.rs",
            component_name.to_case(Case::Snake)
        );

        append_mod(&mod_path, lines)
    }

    pub fn generate(&self, params: DbInfrastructureParams) -> Result<(), String> {
        self.generate_dto(params.clone())?;
        self.logger.info("DTO generated.");
        self.generate_repository(params)?;
        self.logger.info("Repository generated.");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use argentum_log_business::Level;
    use handlebars::Handlebars;
    use std::sync::Arc;

    struct TestLogger;

    impl LoggerTrait for TestLogger {
        fn log<S: AsRef<str>>(&self, _level: Level, _msg: S) {}
    }

    fn test_generator() -> DbInfrastructureGenerator<TestLogger> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("t", "x").unwrap();

        DbInfrastructureGenerator::new(
            Arc::new(TestLogger),
            Arc::new(Renderer::new(Arc::new(handlebars))),
            Arc::new(FileExistenceChecker),
        )
    }

    #[test]
    fn generate_skips_existing_dto_file() {
        let _cwd_guard = crate::generator::test_util::lock_cwd();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let dto_path = temp_dir
            .path()
            .join("user_account/infrastructure/src/db/dto/user_account_dto.rs");
        std::fs::create_dir_all(dto_path.parent().unwrap()).unwrap();
        std::fs::write(&dto_path, "existing").unwrap();

        let result = test_generator().generate(DbInfrastructureParams {
            component_name: "user_account".to_string(),
            entity_name: "UserAccount".to_string(),
        });

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&dto_path).unwrap(), "existing");
    }
}
