use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::cli_params::UiParams;
use crate::generator::FileGenerator;
use crate::generator::ui::ui_data::UiData;

const TEMPLATE: &str = "ui/readme.adoc";

pub(crate) struct UiReadmeGenerator<L>
where
    L: LoggerTrait,
{
    file: Arc<FileGenerator<L>>,
}

impl<L> UiReadmeGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file: Arc<FileGenerator<L>>) -> Self {
        Self { file }
    }

    pub fn generate(&self, params: &UiParams) -> Result<(), String> {
        let data = UiData::new(params.component_name.clone());

        self.file
            .generate(&format!("{}/ui/readme.adoc", data.component_name), TEMPLATE)
    }
}
