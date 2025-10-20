use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

pub(crate) struct FileGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
}

impl<L> FileGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(logger: Arc<L>, renderer: Arc<Renderer>) -> Self {
        Self { logger, renderer }
    }

    pub(crate) fn generate(&self, output_path: &str, template: &str) -> Result<(), String> {
        self.renderer.render("", template, (), output_path)?;

        self.logger.info(format!("{} is generated.", output_path));

        Ok(())
    }
}
