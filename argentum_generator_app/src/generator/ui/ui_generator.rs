use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::cli_params::UiParams;
use crate::generator::ui::{
    UiCargoGenerator, UiLibGenerator, UiPageGenerator, UiPageModGenerator, UiReadmeGenerator,
    UiRsxModGenerator,
};

pub(crate) struct UiGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    cargo: Arc<UiCargoGenerator<L>>,
    readme: Arc<UiReadmeGenerator<L>>,
    lib: Arc<UiLibGenerator<L>>,
    rsx_mod: Arc<UiRsxModGenerator<L>>,
    page_mod: Arc<UiPageModGenerator<L>>,
    page: Arc<UiPageGenerator<L>>,
}

impl<L> UiGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(
        logger: Arc<L>,
        cargo: Arc<UiCargoGenerator<L>>,
        readme: Arc<UiReadmeGenerator<L>>,
        lib: Arc<UiLibGenerator<L>>,
        rsx_mod: Arc<UiRsxModGenerator<L>>,
        page_mod: Arc<UiPageModGenerator<L>>,
        page: Arc<UiPageGenerator<L>>,
    ) -> Self {
        Self {
            logger,
            cargo,
            readme,
            lib,
            rsx_mod,
            page_mod,
            page,
        }
    }

    pub fn generate(&self, params: &UiParams) -> Result<(), String> {
        self.cargo.generate(params)?;
        self.readme.generate(params)?;
        self.lib.generate(params)?;
        self.rsx_mod.generate(params)?;
        self.page_mod.generate(params)?;
        self.page.generate(params)?;

        self.logger.info("UI package is generated.");

        Ok(())
    }
}
