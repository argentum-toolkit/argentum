mod business_cargo_generator;
mod business_di_generator;
mod business_entity_generator;
mod business_generator;
mod business_git_ignore_generator;
mod business_lib_generator;
mod business_mock_generator;
mod business_readme_generator;
mod business_repository_generator;

pub use business_cargo_generator::BusinessCargoGenerator;
pub use business_di_generator::BusinessDiGenerator;
pub use business_entity_generator::BusinessEntityGenerator;
pub use business_generator::BusinessGenerator;
pub use business_git_ignore_generator::BusinessGitIgnoreGenerator;
pub use business_lib_generator::BusinessLibGenerator;
pub use business_mock_generator::BusinessMockGenerator;
pub use business_readme_generator::BusinessReadmeGenerator;
pub use business_repository_generator::BusinessRepositoryGenerator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{FileExistenceChecker, FileGenerator};
    use crate::params::BusinessParams;
    use argentum_log_business::Level;
    use argentum_template_infrastructure::Renderer;
    use handlebars::Handlebars;
    use std::sync::Arc;

    struct TestLogger;

    impl argentum_log_business::LoggerTrait for TestLogger {
        fn log<S: AsRef<str>>(&self, _level: Level, _msg: S) {}
    }

    fn test_business_generator() -> BusinessGenerator<TestLogger> {
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

        BusinessGenerator::new(
            logger.clone(),
            Arc::new(BusinessCargoGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(BusinessGitIgnoreGenerator::new(file_generator.clone())),
            Arc::new(BusinessReadmeGenerator::new(file_generator.clone())),
            Arc::new(BusinessLibGenerator::new(file_generator.clone())),
            Arc::new(BusinessEntityGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(BusinessRepositoryGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(BusinessMockGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(BusinessDiGenerator::new(
                logger,
                renderer,
                file_existence_checker,
            )),
        )
    }

    #[test]
    fn generate_skips_existing_cargo_file() {
        let _cwd_guard = crate::generator::test_util::lock_cwd();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let cargo_path = temp_dir.path().join("test_comp/business/Cargo.toml");
        std::fs::create_dir_all(cargo_path.parent().unwrap()).unwrap();
        std::fs::write(&cargo_path, "existing").unwrap();

        let result = test_business_generator().generate(&BusinessParams {
            component_name: "test_comp".to_string(),
            entity_name: "TestComp".to_string(),
        });

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&cargo_path).unwrap(), "existing");
    }
}
