use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::cli_params::DevopsParams;
use crate::generator::FileExistenceChecker;
use crate::generator::devops::docker_data::DevopsDocker;

const TEMPLATE: &str = "devops/docker-compose.yaml";

pub(crate) struct DockerComposeGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> DockerComposeGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(
        logger: Arc<L>,
        renderer: Arc<Renderer>,
        file_existence_checker: Arc<FileExistenceChecker>,
    ) -> Self {
        Self {
            logger,
            renderer,
            file_existence_checker,
        }
    }

    pub fn generate(&self, params: &DevopsParams) -> Result<(), String> {
        let data = DevopsDocker::new(params.component_name.clone());

        let output_path = format!("{}/devops/docker/docker-compose.yaml", data.component_name);

        self.file_existence_checker.ensure_absent(&output_path)?;

        self.renderer.render("", TEMPLATE, &data, &output_path)?;

        self.logger.info("docker-compose.yaml is generated.");

        Ok(())
    }
}
