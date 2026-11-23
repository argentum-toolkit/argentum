use std::path::Path;
use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use convert_case::{Case, Casing};

use crate::generator::api_app::{
    ApiAppApiDiGenerator, ApiAppApiMainGenerator, ApiAppCargoGenerator, ApiAppMigrationDiGenerator,
    ApiAppMigrationMainGenerator, ApiAppReadmeGenerator,
};
use crate::params::ApiAppParams;

pub struct ApiAppGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    cargo: Arc<ApiAppCargoGenerator<L>>,
    api_main: Arc<ApiAppApiMainGenerator<L>>,
    api_di: Arc<ApiAppApiDiGenerator<L>>,
    migration_main: Arc<ApiAppMigrationMainGenerator<L>>,
    migration_di: Arc<ApiAppMigrationDiGenerator<L>>,
    readme: Arc<ApiAppReadmeGenerator<L>>,
}

impl<L> ApiAppGenerator<L>
where
    L: LoggerTrait,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        logger: Arc<L>,
        cargo: Arc<ApiAppCargoGenerator<L>>,
        api_main: Arc<ApiAppApiMainGenerator<L>>,
        api_di: Arc<ApiAppApiDiGenerator<L>>,
        migration_main: Arc<ApiAppMigrationMainGenerator<L>>,
        migration_di: Arc<ApiAppMigrationDiGenerator<L>>,
        readme: Arc<ApiAppReadmeGenerator<L>>,
    ) -> Self {
        Self {
            logger,
            cargo,
            api_main,
            api_di,
            migration_main,
            migration_di,
            readme,
        }
    }

    pub fn generate(&self, params: &ApiAppParams) -> Result<(), String> {
        let project_prefix = params.project_prefix.to_case(Case::Snake);
        let component_name = params.component_name.to_case(Case::Snake);
        let dir = format!("{project_prefix}_{component_name}");
        let output = format!("{dir}/api-app/");

        let output_depth = Path::new(&output).components().count();
        let argentum_path = "../".repeat(output_depth);
        let argentum_path = argentum_path.trim_end_matches('/').to_string();

        self.logger.info(format!(
            "Generating api-app `{dir}/api-app` (package `{project_prefix}_{component_name}_api_app`)..."
        ));

        self.cargo.generate(params, &argentum_path)?;
        self.api_main.generate(&project_prefix, &component_name)?;
        self.api_di.generate(params, &argentum_path)?;
        self.migration_main
            .generate(&project_prefix, &component_name)?;
        self.migration_di.generate(params, &argentum_path)?;
        self.readme.generate(&project_prefix, &component_name)?;

        self.logger.info("API app is generated.");

        Ok(())
    }
}
