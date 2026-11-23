use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use convert_case::{Case, Casing};

use crate::generator::{
    ApiAppGenerator, ApiDocGenerator, BusinessGenerator, DevopsGenerator, InfrastructureGenerator,
    RestGenerator, UiGenerator,
};
use crate::params::{
    ApiAppParams, ApiDocParams, BusinessParams, ComponentParams, DevopsParams,
    InfrastructureParams, RestParams, UiParams,
};

pub struct ComponentGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    business: Arc<BusinessGenerator<L>>,
    api_doc: Arc<ApiDocGenerator<L>>,
    rest: Arc<RestGenerator<L>>,
    devops: Arc<DevopsGenerator<L>>,
    infrastructure: Arc<InfrastructureGenerator<L>>,
    ui: Arc<UiGenerator<L>>,
    api_app: Arc<ApiAppGenerator<L>>,
}

impl<L> ComponentGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(
        logger: Arc<L>,
        business: Arc<BusinessGenerator<L>>,
        api_doc: Arc<ApiDocGenerator<L>>,
        rest: Arc<RestGenerator<L>>,
        devops: Arc<DevopsGenerator<L>>,
        infrastructure: Arc<InfrastructureGenerator<L>>,
        ui: Arc<UiGenerator<L>>,
        api_app: Arc<ApiAppGenerator<L>>,
    ) -> Self {
        Self {
            logger,
            business,
            api_doc,
            rest,
            devops,
            infrastructure,
            ui,
            api_app,
        }
    }

    pub fn generate(&self, params: &ComponentParams) -> Result<(), String> {
        let project_prefix = params.project_prefix.to_case(Case::Snake);
        let component_name = params.component_name.to_case(Case::Snake);
        let entity_name = params
            .entity_name
            .clone()
            .unwrap_or_else(|| component_name.to_case(Case::Pascal));

        self.logger.info(format!(
            "Generating component `{}_{}` (entity: `{}`)...",
            project_prefix, component_name, entity_name
        ));

        self.business.generate(&BusinessParams {
            project_prefix: project_prefix.clone(),
            component_name: component_name.clone(),
            entity_name: entity_name.clone(),
        })?;

        self.logger.info("Business package is generated.");

        self.api_doc.generate(&ApiDocParams {
            project_prefix: project_prefix.clone(),
            component_name: component_name.clone(),
        })?;

        self.logger.info("Api-doc folder is generated.");

        self.rest.generate(&RestParams {
            project_prefix: project_prefix.clone(),
            component_name: component_name.clone(),
        })?;

        self.logger.info("Rest package is generated.");

        self.devops.generate(&DevopsParams {
            project_prefix: project_prefix.clone(),
            component_name: component_name.clone(),
        })?;

        self.logger.info("Devops folder is generated.");

        self.infrastructure.generate(&InfrastructureParams {
            project_prefix: project_prefix.clone(),
            component_name: component_name.clone(),
            entity_name: entity_name.clone(),
        })?;

        self.logger.info("Infrastructure package is generated.");

        self.ui.generate(&UiParams {
            project_prefix: project_prefix.clone(),
            component_name: component_name.clone(),
        })?;

        self.logger.info("UI package is generated.");

        self.api_app.generate(&ApiAppParams {
            project_prefix: project_prefix.clone(),
            component_name: component_name.clone(),
        })?;

        self.logger.info("API app is generated.");

        self.logger.info(format!(
            "Component `{}_{}` is generated.",
            project_prefix, component_name
        ));

        Ok(())
    }
}
