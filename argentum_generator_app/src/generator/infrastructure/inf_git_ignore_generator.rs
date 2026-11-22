use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{cli_params::InfrastructureParams, generator::FileGenerator};

use convert_case::{Case, Casing};

const TEMPLATE: &str = "infrastructure/.gitignore";

pub(crate) struct InfGitIgnoreGenerator<L>
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
                params.component_name.to_case(Case::Snake)
            ),
            TEMPLATE,
        )
    }
}
