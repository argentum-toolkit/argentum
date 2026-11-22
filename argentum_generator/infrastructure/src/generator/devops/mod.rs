mod devops_generator;
mod docker_compose_dev_generator;
mod docker_compose_generator;
mod docker_data;

pub use devops_generator::DevopsGenerator;
pub use docker_compose_dev_generator::DockerComposeDevGenerator;
pub use docker_compose_generator::DockerComposeGenerator;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::generator::FileExistenceChecker;
    use crate::params::DevopsParams;
    use argentum_log_business::Level;
    use argentum_template_infrastructure::Renderer;
    use handlebars::Handlebars;

    struct TestLogger;

    impl argentum_log_business::LoggerTrait for TestLogger {
        fn log<S: AsRef<str>>(&self, _level: Level, _msg: S) {}
    }

    fn test_devops_generator() -> DevopsGenerator<TestLogger> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("t", "x").unwrap();

        let logger = Arc::new(TestLogger);
        let renderer = Arc::new(Renderer::new(Arc::new(handlebars)));
        let file_existence_checker = Arc::new(FileExistenceChecker);

        DevopsGenerator::new(
            logger.clone(),
            Arc::new(DockerComposeGenerator::new(
                logger.clone(),
                renderer.clone(),
                file_existence_checker.clone(),
            )),
            Arc::new(DockerComposeDevGenerator::new(
                logger,
                renderer,
                file_existence_checker,
            )),
        )
    }

    #[test]
    fn generate_does_not_overwrite_existing_docker_compose_file() {
        let _cwd_guard = crate::generator::test_util::lock_cwd();
        let temp_dir = tempfile::tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let compose_path = temp_dir
            .path()
            .join("test_comp/devops/docker/docker-compose.yaml");
        std::fs::create_dir_all(compose_path.parent().unwrap()).unwrap();
        std::fs::write(&compose_path, "existing").unwrap();

        let result = test_devops_generator().generate(&DevopsParams {
            component_name: "test_comp".to_string(),
        });

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
        assert_eq!(std::fs::read_to_string(&compose_path).unwrap(), "existing");
    }
}
