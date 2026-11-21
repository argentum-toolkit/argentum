use std::io::Write;
use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::cli_params::DbInfrastructureParams;
use crate::generator::FileExistenceChecker;
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

    fn append_mod(&self, mod_path: &str, lines: Vec<String>) -> Result<(), String> {
        let content = if std::path::Path::new(mod_path).exists() {
            std::fs::read_to_string(mod_path).map_err(|e| e.to_string())?
        } else {
            String::new()
        };

        let existing: Vec<&str> = content.lines().map(str::trim).collect();

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(mod_path)
            .map_err(|e| e.to_string())?;

        for line in lines {
            if !existing.contains(&line.trim()) {
                writeln!(file, "{}", line).map_err(|e| e.to_string())?
            }
        }

        Ok(())
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

        self.file_existence_checker.ensure_absent(&output_path)?;

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

        self.append_mod(&mod_path, lines)
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

        self.file_existence_checker.ensure_absent(&output_path)?;

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

        self.append_mod(&mod_path, lines)
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
    fn append_mod_creates_file_and_writes_lines() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("mod.rs");
        let path_str = path.to_str().unwrap();

        test_generator()
            .append_mod(path_str, vec!["mod skill_dto;".to_string()])
            .unwrap();

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "mod skill_dto;\n");
    }

    #[test]
    fn append_mod_does_not_duplicate_lines() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("mod.rs");
        let path_str = path.to_str().unwrap();
        let lines = vec![
            "mod skill_dto;".to_string(),
            "pub use skill_dto::SkillDto;".to_string(),
        ];

        let generator = test_generator();
        generator.append_mod(path_str, lines.clone()).unwrap();
        generator.append_mod(path_str, lines).unwrap();

        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "mod skill_dto;\npub use skill_dto::SkillDto;\n"
        );
    }

    #[test]
    fn append_mod_does_not_skip_line_substring_of_existing_line() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("mod.rs");
        let path_str = path.to_str().unwrap();
        std::fs::write(&path, "pub use user_repository::UserRepositoryFactory;\n").unwrap();

        test_generator()
            .append_mod(
                path_str,
                vec!["pub use user_repository::UserRepository;".to_string()],
            )
            .unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("pub use user_repository::UserRepositoryFactory;"));
        assert!(content.contains("pub use user_repository::UserRepository;\n"));
    }

    #[test]
    fn append_mod_tolerates_whitespace_differences() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("mod.rs");
        let path_str = path.to_str().unwrap();
        std::fs::write(&path, "  mod skill_dto;  \n").unwrap();

        test_generator()
            .append_mod(path_str, vec!["mod skill_dto;".to_string()])
            .unwrap();

        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "  mod skill_dto;  \n"
        );
    }

    #[test]
    fn generate_does_not_overwrite_existing_dto_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let dto_path = temp_dir
            .path()
            .join("pro_pro/infrastructure/src/db/dto/skill_dto.rs");
        std::fs::create_dir_all(dto_path.parent().unwrap()).unwrap();
        std::fs::write(&dto_path, "existing").unwrap();

        let result = test_generator().generate(DbInfrastructureParams {
            component_name: "pro_pro".to_string(),
            entity_name: "Skill".to_string(),
        });

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert_eq!(std::fs::read_to_string(&dto_path).unwrap(), "existing");
    }
}
