use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{generator::FileGenerator, params::FeAppParams};

use convert_case::{Case, Casing};

const INPUT_CSS_TEMPLATE: &str = "fe_app/input.css";
const PACKAGE_JSON_TEMPLATE: &str = "fe_app/package.json";
const TAILWIND_CONFIG_JS_TEMPLATE: &str = "fe_app/tailwind.config.js";

pub struct FeStyleGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    file: Arc<FileGenerator<L>>,
}

impl<L> FeStyleGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(logger: Arc<L>, file: Arc<FileGenerator<L>>) -> Self {
        Self { logger, file }
    }

    pub fn generate(&self, params: &FeAppParams) -> Result<(), String> {
        let app_name = params.app_name.to_case(Case::Snake);

        self.file
            .generate(&format!("{}/input.css", app_name), INPUT_CSS_TEMPLATE)?;

        self.file
            .generate(&format!("{}/package.json", app_name), PACKAGE_JSON_TEMPLATE)?;

        self.file.generate(
            &format!("{}/tailwind.config.js", app_name),
            TAILWIND_CONFIG_JS_TEMPLATE,
        )?;

        self.logger.info("Files for styles support are generated");

        Ok(())
    }
}
