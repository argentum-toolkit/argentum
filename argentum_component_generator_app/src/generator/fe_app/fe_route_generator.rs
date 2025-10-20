use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{cli_params::FeAppParams, generator::FileGenerator};

use convert_case::{Case, Casing};

const TEMPLATE: &str = "fe_app/src/route";

pub(crate) struct FeRouteGenerator<L>
where
    L: LoggerTrait,
{
    file: Arc<FileGenerator<L>>,
}

impl<L> FeRouteGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file: Arc<FileGenerator<L>>) -> Self {
        Self { file }
    }

    pub fn generate(&self, params: &FeAppParams) -> Result<(), String> {
        self.file.generate(
            &format!("{}/src/route.rs", params.app_name.to_case(Case::Snake)),
            TEMPLATE,
        )
    }
}
