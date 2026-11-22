mod db_infrastructure;
mod inf_adapter_generator;
mod inf_cargo_generator;
mod inf_db_mod_generator;
mod inf_di_generator;
mod inf_git_ignore_generator;
mod inf_lib_generator;
mod inf_migration_generator;
mod inf_readme_generator;
mod infrastructure_generator;

pub(crate) use db_infrastructure::DbInfrastructureGenerator;
pub(crate) use inf_adapter_generator::InfAdapterGenerator;
pub(crate) use inf_cargo_generator::InfCargoGenerator;
pub(crate) use inf_db_mod_generator::InfDbModGenerator;
pub(crate) use inf_di_generator::InfDiGenerator;
pub(crate) use inf_git_ignore_generator::InfGitIgnoreGenerator;
pub(crate) use inf_lib_generator::InfLibGenerator;
pub(crate) use inf_migration_generator::InfMigrationGenerator;
pub(crate) use inf_readme_generator::InfReadmeGenerator;
pub(crate) use infrastructure_generator::InfrastructureGenerator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli_params::InfrastructureParams;
    use crate::generator::{FileExistenceChecker, FileGenerator};
    use argentum_log_business::Level;
    use argentum_template_infrastructure::Renderer;
    use handlebars::Handlebars;
    use std::sync::Arc;

    struct TestLogger;

    impl argentum_log_business::LoggerTrait for TestLogger {
        fn log<S: AsRef<str>>(&self, _level: Level, _msg: S) {}
    }

    fn test_infrastructure_generator() -> InfrastructureGenerator<TestLogger> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("t", "x").unwrap();

        let logger = Arc::new(TestLogger);
        let renderer = Arc::new(Renderer::new(Arc::new(handlebars)));
        let file_existence_checker = Arc::new(FileExistenceChecker);
        let file_generator = Arc::new(FileGenerator::new(
            logger.clone(),
            renderer.clone(),
            file_existence_checker.clone(),
        ));

        InfrastructureGenerator::new(
            logger.clone(),
            Arc::new(InfCargoGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(InfGitIgnoreGenerator::new(file_generator.clone())),
            Arc::new(InfReadmeGenerator::new(file_generator.clone())),
            Arc::new(InfLibGenerator::new(file_generator.clone())),
            Arc::new(InfDiGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(InfDbModGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(InfAdapterGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(InfMigrationGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(DbInfrastructureGenerator::new(
                logger,
                renderer,
                file_existence_checker,
            )),
        )
    }

    #[test]
    fn generate_does_not_overwrite_existing_cargo_file() {
        let _cwd_guard = crate::generator::test_util::lock_cwd();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let cargo_path = temp_dir.path().join("test_comp/infrastructure/Cargo.toml");
        std::fs::create_dir_all(cargo_path.parent().unwrap()).unwrap();
        std::fs::write(&cargo_path, "existing").unwrap();

        let result = test_infrastructure_generator().generate(&InfrastructureParams {
            component_name: "test_comp".to_string(),
            entity_name: "TestComp".to_string(),
        });

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert_eq!(std::fs::read_to_string(&cargo_path).unwrap(), "existing");
    }
}
