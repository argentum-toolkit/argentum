use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{cli_params::BusinessParams, generator::FileGenerator};

use convert_case::{Case, Casing};

const TEMPLATE: &str = "business/.gitignore";

pub(crate) struct BusinessGitIgnoreGenerator<L>
where
    L: LoggerTrait,
{
    file: Arc<FileGenerator<L>>,
}

impl<L> BusinessGitIgnoreGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file: Arc<FileGenerator<L>>) -> Self {
        Self { file }
    }

    pub fn generate(&self, params: &BusinessParams) -> Result<(), String> {
        self.file.generate(
            &format!(
                "{}/business/.gitignore",
                params.component_name.to_case(Case::Snake)
            ),
            TEMPLATE,
        )
    }
}
