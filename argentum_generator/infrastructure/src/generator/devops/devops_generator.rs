use std::sync::Arc;

use argentum_log_business::LoggerTrait;

use crate::generator::devops::{DockerComposeDevGenerator, DockerComposeGenerator};
use crate::params::DevopsParams;

pub struct DevopsGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    docker_compose: Arc<DockerComposeGenerator<L>>,
    docker_compose_dev: Arc<DockerComposeDevGenerator<L>>,
}

impl<L> DevopsGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(
        logger: Arc<L>,
        docker_compose: Arc<DockerComposeGenerator<L>>,
        docker_compose_dev: Arc<DockerComposeDevGenerator<L>>,
    ) -> Self {
        Self {
            logger,
            docker_compose,
            docker_compose_dev,
        }
    }

    pub fn generate(&self, params: &DevopsParams) -> Result<(), String> {
        self.docker_compose.generate(params)?;
        self.docker_compose_dev.generate(params)?;

        self.logger.info("Devops folder is generated.");

        Ok(())
    }
}
