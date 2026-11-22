use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{generator::FileGenerator, params::BusinessParams};

use convert_case::{Case, Casing};

const TEMPLATE: &str = "business/readme.adoc";

pub struct BusinessReadmeGenerator<L>
where
    L: LoggerTrait,
{
    file: Arc<FileGenerator<L>>,
}

impl<L> BusinessReadmeGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file: Arc<FileGenerator<L>>) -> Self {
        Self { file }
    }

    pub fn generate(&self, params: &BusinessParams) -> Result<(), String> {
        self.file.generate(
            &format!(
                "{}/business/readme.adoc",
                params.component_name.to_case(Case::Snake)
            ),
            TEMPLATE,
        )
    }
}
