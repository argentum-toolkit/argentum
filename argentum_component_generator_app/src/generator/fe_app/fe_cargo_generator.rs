use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::cli_params::FeAppParams;

use convert_case::{Case, Casing};
use serde::Serialize;

const CARGO_TEMPLATE: &str = "fe_app/Cargo.toml";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    app_name: String,

    argentum_path: Option<String>,
}

pub(crate) struct FeCargoGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
}

impl<L> FeCargoGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(logger: Arc<L>, renderer: Arc<Renderer>) -> Self {
        Self { logger, renderer }
    }

    pub fn generate(
        &self,
        params: &FeAppParams,
        argentum_path: Option<String>,
    ) -> Result<(), String> {
        let app_name = params.app_name.clone();

        let output_path = format!("{}/Cargo.toml", app_name.to_case(Case::Snake),);

        let data = Data {
            app_name: app_name.to_case(Case::Kebab),
            argentum_path,
        };

        self.renderer
            .render("", CARGO_TEMPLATE, &data, &output_path)?;

        self.logger.info("Cargo.toml is generated.");

        Ok(())
    }
}
