use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::generator::FileGenerator;
use crate::generator::ui::ui_data::UiData;
use crate::params::UiParams;

const TEMPLATE: &str = "ui/src/rsx/mod";

pub struct UiRsxModGenerator<L>
where
    L: LoggerTrait,
{
    file: Arc<FileGenerator<L>>,
}

impl<L> UiRsxModGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file: Arc<FileGenerator<L>>) -> Self {
        Self { file }
    }

    pub fn generate(&self, params: &UiParams) -> Result<(), String> {
        let data = UiData::new(params.component_name.clone());

        self.file.generate(
            &format!("{}/ui/src/rsx/mod.rs", data.component_name),
            TEMPLATE,
        )
    }
}
