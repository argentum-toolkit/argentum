use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::Renderer;

use crate::generator::FileExistenceChecker;
use crate::generator::devops::docker_data::DevopsDocker;
use crate::params::DevopsParams;

const TEMPLATE: &str = "devops/docker-compose.dev.yaml";

pub struct DockerComposeDevGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    renderer: Arc<Renderer>,
    file_existence_checker: Arc<FileExistenceChecker>,
}

impl<L> DockerComposeDevGenerator<L>
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
        let data = DevopsDocker::new(params.project_prefix.clone(), params.component_name.clone());

        let output_path = format!(
            "{}_{}/devops/docker/docker-compose.dev.yaml",
            data.project_prefix, data.component_name
        );

        if self.file_existence_checker.exists(&output_path) {
            self.logger
                .info(format!("{output_path} already exists, skipped."));

            return Ok(());
        }

        self.renderer.render("", TEMPLATE, &data, &output_path)?;

        self.logger.info("docker-compose.dev.yaml is generated.");

        Ok(())
    }
}
