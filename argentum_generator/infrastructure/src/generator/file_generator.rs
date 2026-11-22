use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::generator::FileExistenceChecker;

pub struct FileGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> FileGenerator<L>
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

    pub fn generate(&self, output_path: &str, template: &str) -> Result<(), String> {
        if self.file_existence_checker.exists(output_path) {
            self.logger
                .info(format!("{output_path} already exists, skipped."));

            return Ok(());
        }

        self.renderer.render("", template, (), output_path)?;

        self.logger.info(format!("{} is generated.", output_path));

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

    fn test_file_generator() -> FileGenerator<TestLogger> {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("t", "rendered")
            .unwrap();

        FileGenerator::new(
            Arc::new(TestLogger),
            Arc::new(Renderer::new(Arc::new(handlebars))),
            Arc::new(FileExistenceChecker),
        )
    }

    #[test]
    fn generate_skips_existing_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("output.rs");
        std::fs::write(&path, "existing").unwrap();

        let result = test_file_generator().generate(path.to_str().unwrap(), "t");

        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "existing");
    }
}
