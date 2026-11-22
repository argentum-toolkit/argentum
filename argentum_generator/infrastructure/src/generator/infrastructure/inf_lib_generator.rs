use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{generator::FileGenerator, params::InfrastructureParams};

use convert_case::{Case, Casing};

const TEMPLATE: &str = "infrastructure/src/lib";

pub struct InfLibGenerator<L>
where
    L: LoggerTrait,
{
    file: Arc<FileGenerator<L>>,
}

impl<L> InfLibGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(file: Arc<FileGenerator<L>>) -> Self {
        Self { file }
    }

    pub fn generate(&self, params: &InfrastructureParams) -> Result<(), String> {
        self.file.generate(
            &format!(
                "{}/infrastructure/src/lib.rs",
                params.component_name.to_case(Case::Snake)
            ),
            TEMPLATE,
        )
    }
}
