use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::generator::FileExistenceChecker;
use crate::generator::ui::ui_data::UiData;
use crate::params::UiParams;

const TEMPLATE: &str = "ui/src/rsx/page/mod";

pub struct UiPageModGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> UiPageModGenerator<L>
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

    pub fn generate(&self, params: &UiParams) -> Result<(), String> {
        let data = UiData::new(params.component_name.clone());

        let output_path = format!("{}/ui/src/rsx/page/mod.rs", data.component_name);

        self.file_existence_checker.ensure_absent(&output_path)?;

        self.renderer.render("", TEMPLATE, &data, &output_path)?;

        self.logger.info("rsx page mod.rs is generated.");

        Ok(())
    }
}
