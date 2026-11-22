pub mod api_doc_data;
mod api_doc_generator;
mod open_api_generator;
mod paths_generator;
mod request_generator;
mod response_generator;
mod type_generator;

pub use api_doc_generator::ApiDocGenerator;
pub use open_api_generator::OpenApiGenerator;
pub use paths_generator::PathsGenerator;
pub use request_generator::RequestGenerator;
pub use response_generator::ResponseGenerator;
pub use type_generator::TypeGenerator;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::generator::FileExistenceChecker;
    use crate::params::ApiDocParams;
    use argentum_log_business::Level;
    use argentum_template_infrastructure::Renderer;
    use handlebars::Handlebars;

    struct TestLogger;

    impl argentum_log_business::LoggerTrait for TestLogger {
        fn log<S: AsRef<str>>(&self, _level: Level, _msg: S) {}
    }

    fn test_api_doc_generator() -> ApiDocGenerator<TestLogger> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("t", "x").unwrap();

        let logger = Arc::new(TestLogger);
        let renderer = Arc::new(Renderer::new(Arc::new(handlebars)));
        let file_existence_checker = Arc::new(FileExistenceChecker);

        ApiDocGenerator::new(
            logger.clone(),
            Arc::new(OpenApiGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(PathsGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(RequestGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(ResponseGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(TypeGenerator::new(logger, renderer, file_existence_checker)),
        )
    }

    #[test]
    fn generate_skips_existing_openapi_file() {
        let _cwd_guard = crate::generator::test_util::lock_cwd();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let openapi_path = temp_dir.path().join("test_comp/api-doc/openapi.yaml");
        std::fs::create_dir_all(openapi_path.parent().unwrap()).unwrap();
        std::fs::write(&openapi_path, "existing").unwrap();

        let result = test_api_doc_generator().generate(&ApiDocParams {
            component_name: "test_comp".to_string(),
        });

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&openapi_path).unwrap(), "existing");
    }
}
