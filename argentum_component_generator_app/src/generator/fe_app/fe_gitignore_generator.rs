use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{cli_params::FeAppParams, generator::FileGenerator};

use convert_case::{Case, Casing};

const TEMPLATE: &str = "fe_app/.gitignore";

pub(crate) struct FeGitIgnoreGenerator<L>
where
    L: LoggerTrait,
{
    file: Arc<FileGenerator<L>>,
}

impl<L> FeGitIgnoreGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file: Arc<FileGenerator<L>>) -> Self {
        Self { file }
    }

    pub fn generate(&self, params: &FeAppParams) -> Result<(), String> {
        self.file.generate(
            &format!("{}/.gitignore", params.app_name.to_case(Case::Snake)),
            TEMPLATE,
        )
    }
}
