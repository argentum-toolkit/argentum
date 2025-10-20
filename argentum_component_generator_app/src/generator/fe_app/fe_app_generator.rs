use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::{
    cli_params::FeAppParams,
    generator::fe_app::{
        FeGitIgnoreGenerator, FeMainGenerator, FeReadmeAdocGenerator, FeRouteGenerator,
        FeRsxGenerator, FeStyleGenerator, fe_cargo_generator::FeCargoGenerator,
    },
};

pub(crate) struct FeAppGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    cargo: Arc<FeCargoGenerator<L>>,
    style: Arc<FeStyleGenerator<L>>,
    gitignore: Arc<FeGitIgnoreGenerator<L>>,
    readme: Arc<FeReadmeAdocGenerator<L>>,
    rsx: Arc<FeRsxGenerator<L>>,
    main: Arc<FeMainGenerator<L>>,
    route: Arc<FeRouteGenerator<L>>,
}

impl<L> FeAppGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(
        logger: Arc<L>,
        cargo: Arc<FeCargoGenerator<L>>,
        style: Arc<FeStyleGenerator<L>>,
        gitignore: Arc<FeGitIgnoreGenerator<L>>,
        readme: Arc<FeReadmeAdocGenerator<L>>,
        rsx: Arc<FeRsxGenerator<L>>,
        main: Arc<FeMainGenerator<L>>,
        route: Arc<FeRouteGenerator<L>>,
    ) -> Self {
        Self {
            logger,
            cargo,
            style,
            gitignore,
            readme,
            rsx,
            main,
            route,
        }
    }

    pub fn generate(
        &self,
        params: &FeAppParams,
        argentum_path: Option<String>,
    ) -> Result<(), String> {
        self.cargo.generate(params, argentum_path)?;
        self.style.generate(params)?;
        self.gitignore.generate(params)?;
        self.readme.generate(params)?;
        self.rsx.generate(params)?;
        self.main.generate(params)?;
        self.route.generate(params)?;

        self.logger.info("Front-end application is generated.");

        Ok(())
    }
}
