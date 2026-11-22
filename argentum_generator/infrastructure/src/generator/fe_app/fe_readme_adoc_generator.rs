use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{generator::FileGenerator, params::FeAppParams};

use convert_case::{Case, Casing};

const TEMPLATE: &str = "fe_app/readme.adoc";

pub struct FeReadmeAdocGenerator<L>
where
    L: LoggerTrait,
{
    file: Arc<FileGenerator<L>>,
}

impl<L> FeReadmeAdocGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file: Arc<FileGenerator<L>>) -> Self {
        Self { file }
    }

    pub fn generate(&self, params: &FeAppParams) -> Result<(), String> {
        self.file.generate(
            &format!("{}/readme.adoc", params.app_name.to_case(Case::Snake)),
            TEMPLATE,
        )
    }
}
