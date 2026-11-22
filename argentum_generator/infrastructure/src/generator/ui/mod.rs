mod ui_cargo_generator;
pub mod ui_data;
mod ui_generator;
mod ui_lib_generator;
mod ui_page_generator;
mod ui_page_mod_generator;
mod ui_readme_generator;
mod ui_rsx_mod_generator;

pub use ui_cargo_generator::UiCargoGenerator;
pub use ui_generator::UiGenerator;
pub use ui_lib_generator::UiLibGenerator;
pub use ui_page_generator::UiPageGenerator;
pub use ui_page_mod_generator::UiPageModGenerator;
pub use ui_readme_generator::UiReadmeGenerator;
pub use ui_rsx_mod_generator::UiRsxModGenerator;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{FileExistenceChecker, FileGenerator};
    use crate::params::UiParams;
    use argentum_log_business::Level;
    use argentum_template_infrastructure::Renderer;
    use handlebars::Handlebars;
    use std::sync::Arc;

    struct TestLogger;

    impl argentum_log_business::LoggerTrait for TestLogger {
        fn log<S: AsRef<str>>(&self, _level: Level, _msg: S) {}
    }

    fn test_ui_generator() -> UiGenerator<TestLogger> {
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

        UiGenerator::new(
            logger.clone(),
            Arc::new(UiCargoGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(UiReadmeGenerator::new(file_generator.clone())),
            Arc::new(UiLibGenerator::new(file_generator.clone())),
            Arc::new(UiRsxModGenerator::new(file_generator.clone())),
            Arc::new(UiPageModGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(UiPageGenerator::new(
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

        let cargo_path = temp_dir.path().join("test_comp/ui/Cargo.toml");
        std::fs::create_dir_all(cargo_path.parent().unwrap()).unwrap();
        std::fs::write(&cargo_path, "existing").unwrap();

        let result = test_ui_generator().generate(&UiParams {
            component_name: "test_comp".to_string(),
        });

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&cargo_path).unwrap(), "existing");
    }
}
