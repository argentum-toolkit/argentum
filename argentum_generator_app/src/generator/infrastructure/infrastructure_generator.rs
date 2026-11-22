use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::cli_params::{DbInfrastructureParams, InfrastructureParams};
use crate::generator::DbInfrastructureGenerator;
use crate::generator::infrastructure::{
    InfAdapterGenerator, InfCargoGenerator, InfDbModGenerator, InfDiGenerator,
    InfGitIgnoreGenerator, InfLibGenerator, InfMigrationGenerator, InfReadmeGenerator,
};

pub(crate) struct InfrastructureGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    cargo: Arc<InfCargoGenerator<L>>,
    git_ignore: Arc<InfGitIgnoreGenerator<L>>,
    readme: Arc<InfReadmeGenerator<L>>,
    lib: Arc<InfLibGenerator<L>>,
    di: Arc<InfDiGenerator<L>>,
    db_mod: Arc<InfDbModGenerator<L>>,
    adapter: Arc<InfAdapterGenerator<L>>,
    migration: Arc<InfMigrationGenerator<L>>,
    db_infrastructure: Arc<DbInfrastructureGenerator<L>>,
}

impl<L> InfrastructureGenerator<L>
where
    L: LoggerTrait,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        logger: Arc<L>,
        cargo: Arc<InfCargoGenerator<L>>,
        git_ignore: Arc<InfGitIgnoreGenerator<L>>,
        readme: Arc<InfReadmeGenerator<L>>,
        lib: Arc<InfLibGenerator<L>>,
        di: Arc<InfDiGenerator<L>>,
        db_mod: Arc<InfDbModGenerator<L>>,
        adapter: Arc<InfAdapterGenerator<L>>,
        migration: Arc<InfMigrationGenerator<L>>,
        db_infrastructure: Arc<DbInfrastructureGenerator<L>>,
    ) -> Self {
        Self {
            logger,
            cargo,
            git_ignore,
            readme,
            lib,
            di,
            db_mod,
            adapter,
            migration,
            db_infrastructure,
        }
    }

    pub fn generate(&self, params: &InfrastructureParams) -> Result<(), String> {
        self.cargo.generate(params)?;
        self.git_ignore.generate(params)?;
        self.readme.generate(params)?;
        self.lib.generate(params)?;
        self.di.generate(params)?;
        self.db_mod.generate(params)?;
        self.adapter.generate(params)?;
        self.migration.generate(params)?;

        self.db_infrastructure.generate(DbInfrastructureParams {
            component_name: params.component_name.clone(),
            entity_name: params.entity_name.clone(),
        })?;

        self.logger.info("Infrastructure package is generated.");

        Ok(())
    }
}
