use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{cli_params::BusinessParams, generator::FileGenerator};

use convert_case::{Case, Casing};

const TEMPLATE: &str = "business/src/lib";

pub(crate) struct BusinessLibGenerator<L>
where
    L: LoggerTrait,
{
    file: Arc<FileGenerator<L>>,
}

impl<L> BusinessLibGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file: Arc<FileGenerator<L>>) -> Self {
        Self { file }
    }

    pub fn generate(&self, params: &BusinessParams) -> Result<(), String> {
        self.file.generate(
            &format!(
                "{}/business/src/lib.rs",
                params.component_name.to_case(Case::Snake)
            ),
            TEMPLATE,
        )
    }
}
