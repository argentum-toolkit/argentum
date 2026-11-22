use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{cli_params::FeAppParams, generator::file_generator::FileGenerator};

use convert_case::{Case, Casing};

const ELEMENT_MOD_TEMPLATE: &str = "fe_app/src/rsx/element_mod";
const PAGE_MOD_TEMPLATE: &str = "fe_app/src/rsx/page_mod";
const APP_TEMPLATE: &str = "fe_app/src/rsx/app";
const MOD_TEMPLATE: &str = "fe_app/src/rsx/mod";
const WRAPPER_TEMPLATE: &str = "fe_app/src/rsx/wrapper";

pub(crate) struct FeRsxGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    file: Arc<FileGenerator<L>>,
}

impl<L> FeRsxGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(logger: Arc<L>, file: Arc<FileGenerator<L>>) -> Self {
        Self { logger, file }
    }

    pub(crate) fn generate(&self, params: &FeAppParams) -> Result<(), String> {
        let app_name = params.app_name.to_case(Case::Snake);

        self.file.generate(
            &format!("{}/src/rsx/element/mod.rs", app_name),
            ELEMENT_MOD_TEMPLATE,
        )?;

        self.file.generate(
            &format!("{}/src/rsx/page/mod.rs", app_name),
            PAGE_MOD_TEMPLATE,
        )?;

        self.file
            .generate(&format!("{}/src/rsx/mod.rs", app_name), MOD_TEMPLATE)?;
        self.file
            .generate(&format!("{}/src/rsx/app.rs", app_name), APP_TEMPLATE)?;
        self.file.generate(
            &format!("{}/src/rsx/wrapper.rs", app_name),
            WRAPPER_TEMPLATE,
        )?;

        self.logger.info("rsx is generated.");

        Ok(())
    }
}
