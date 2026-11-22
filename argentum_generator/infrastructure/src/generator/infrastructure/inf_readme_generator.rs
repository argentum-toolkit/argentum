use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{generator::FileGenerator, params::InfrastructureParams};

use convert_case::{Case, Casing};

const TEMPLATE: &str = "infrastructure/readme.adoc";

pub struct InfReadmeGenerator<L>
where
    L: LoggerTrait,
{
    file: Arc<FileGenerator<L>>,
}

impl<L> InfReadmeGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file: Arc<FileGenerator<L>>) -> Self {
        Self { file }
    }

    pub fn generate(&self, params: &InfrastructureParams) -> Result<(), String> {
        self.file.generate(
            &format!(
                "{}/infrastructure/readme.adoc",
                format!(
                    "{}_{}",
                    params.project_prefix.to_case(Case::Snake),
                    params.component_name.to_case(Case::Snake)
                )
            ),
            TEMPLATE,
        )
    }
}
