use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{generator::FileGenerator, params::InfrastructureParams};

use convert_case::{Case, Casing};

const TEMPLATE: &str = "infrastructure/.gitignore";

pub struct InfGitIgnoreGenerator<L>
where
    L: LoggerTrait,
{
    file: Arc<FileGenerator<L>>,
}

impl<L> InfGitIgnoreGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file: Arc<FileGenerator<L>>) -> Self {
        Self { file }
    }

    pub fn generate(&self, params: &InfrastructureParams) -> Result<(), String> {
        self.file.generate(
            &format!(
                "{}/infrastructure/.gitignore",
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
