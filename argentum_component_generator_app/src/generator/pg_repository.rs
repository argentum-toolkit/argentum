use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::cli_params::PgRepositoryParams;

pub(crate) struct PgRepositoryGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
}

impl<L> PgRepositoryGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(logger: Arc<L>, renderer: Arc<Renderer>) -> Self {
        Self { logger, renderer }
    }

    pub fn generate(&self, params: PgRepositoryParams) -> Result<(), String> {
        Ok(())
    }
}
