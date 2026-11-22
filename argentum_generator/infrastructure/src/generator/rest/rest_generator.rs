use std::io::ErrorKind;
use std::path::Path;
use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_rest_generator_infrastructure::di::di_factory as rest_di_factory;
use argentum_rest_generator_infrastructure::generator_params::GeneratorParams;
use convert_case::{Case, Casing};

use crate::params::RestParams;

pub struct RestGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
}

impl<L> RestGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(logger: Arc<L>) -> Self {
        Self { logger }
    }

    pub fn generate(&self, params: &RestParams) -> Result<(), String> {
        let component_name = params.component_name.to_case(Case::Snake);
        let output = format!("{component_name}/rest/");
        let input = format!("{component_name}/api-doc/openapi.yaml");
        let package_name = format!("argentum_{component_name}_rest");

        if !Path::new(&input).is_file() {
            return Err(format!(
                "Input OpenAPI specification `{input}` does not exist. Generate the api-doc folder for `{component_name}` first."
            ));
        }

        for stale_dir in ["src", "api-spec"] {
            let stale_path = format!("{output}{stale_dir}");
            match std::fs::remove_dir_all(&stale_path) {
                Ok(()) => self
                    .logger
                    .info(format!("Removed stale generated folder `{stale_path}`.")),
                Err(e) if e.kind() == ErrorKind::NotFound => {}
                Err(e) => return Err(format!("Failed to remove `{stale_path}`: {e}")),
            }
        }

        let output_depth = Path::new(&output).components().count();
        let argentum_path = "../".repeat(output_depth);
        let argentum_path = argentum_path.trim_end_matches('/').to_string();

        self.logger.info(format!(
            "Generating rest package `{package_name}` from `{input}`..."
        ));

        let rest_di = rest_di_factory(self.logger.clone())
            .map_err(|e| format!("Failed to build the rest generator: {e}"))?;

        rest_di
            .openapi_generator
            .generate(GeneratorParams {
                input,
                output,
                package_name,
                homepage: None,
                repository: None,
                documentation: None,
                argentum_path: Some(argentum_path),
            })
            .map_err(|e| format!("Rest generation failed: {e}"))?;

        self.logger.info("Rest package is generated.");

        Ok(())
    }
}
