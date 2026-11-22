use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use convert_case::{Case, Casing};

use crate::generator::{
    ApiDocGenerator, BusinessGenerator, DevopsGenerator, InfrastructureGenerator, UiGenerator,
};
use crate::params::{
    ApiDocParams, BusinessParams, ComponentParams, DevopsParams, InfrastructureParams, UiParams,
};

pub struct ComponentGenerator<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
    business: Arc<BusinessGenerator<L>>,
    api_doc: Arc<ApiDocGenerator<L>>,
    devops: Arc<DevopsGenerator<L>>,
    infrastructure: Arc<InfrastructureGenerator<L>>,
    ui: Arc<UiGenerator<L>>,
}

impl<L> ComponentGenerator<L>
where
    L: LoggerTrait,
{
    pub fn new(
        logger: Arc<L>,
        business: Arc<BusinessGenerator<L>>,
        api_doc: Arc<ApiDocGenerator<L>>,
        devops: Arc<DevopsGenerator<L>>,
        infrastructure: Arc<InfrastructureGenerator<L>>,
        ui: Arc<UiGenerator<L>>,
    ) -> Self {
        Self {
            logger,
            business,
            api_doc,
            devops,
            infrastructure,
            ui,
        }
    }

    pub fn generate(&self, params: &ComponentParams) -> Result<(), String> {
        let component_name = params.component_name.to_case(Case::Snake);
        let entity_name = params
            .entity_name
            .clone()
            .unwrap_or_else(|| component_name.to_case(Case::Pascal));

        self.logger.info(format!(
            "Generating component `{}` (entity: `{}`)...",
            component_name, entity_name
        ));

        self.business.generate(&BusinessParams {
            component_name: component_name.clone(),
            entity_name: entity_name.clone(),
        })?;

        self.logger.info("Business package is generated.");

        self.api_doc.generate(&ApiDocParams {
            component_name: component_name.clone(),
        })?;

        self.logger.info("Api-doc folder is generated.");

        self.devops.generate(&DevopsParams {
            component_name: component_name.clone(),
        })?;

        self.logger.info("Devops folder is generated.");

        self.infrastructure.generate(&InfrastructureParams {
            component_name: component_name.clone(),
            entity_name: entity_name.clone(),
        })?;

        self.logger.info("Infrastructure package is generated.");

        self.ui.generate(&UiParams {
            component_name: component_name.clone(),
        })?;

        self.logger.info("UI package is generated.");

        self.logger
            .info(format!("Component `{}` is generated.", component_name));

        Ok(())
    }
}
