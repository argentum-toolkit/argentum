use argentum_generator_infrastructure::di::di_factory;
use argentum_generator_infrastructure::params::{
    ApiDocParams, BusinessParams, ComponentParams, DbInfrastructureParams, DevopsParams,
    FeAppParams, InfrastructureParams, UiParams,
};
use argentum_log_business::{DefaultLogger, Level};
use argentum_log_infrastructure::stdout::PrettyWriter;
use clap::Parser;
use std::error::Error;
use std::sync::Arc;

mod cli_params;

use crate::cli_params::{CliParams, GenerateCommand};

fn main() -> Result<(), Box<dyn Error>> {
    let cli: CliParams = CliParams::parse();

    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let di = di_factory(logger)?;

    match cli.command {
        GenerateCommand::ApiDoc { component_name } => {
            let params = ApiDocParams { component_name };
            Ok(di.api_doc_generator.generate(&params)?)
        }
        GenerateCommand::Business {
            component_name,
            entity_name,
        } => {
            let params = BusinessParams {
                component_name,
                entity_name,
            };
            Ok(di.business_generator.generate(&params)?)
        }
        GenerateCommand::Component {
            component_name,
            entity_name,
        } => {
            let params = ComponentParams {
                component_name,
                entity_name,
            };
            Ok(di.component_generator.generate(&params)?)
        }
        GenerateCommand::DbInfrastructure {
            component_name,
            entity_name,
        } => {
            let params = DbInfrastructureParams {
                entity_name,
                component_name,
            };
            Ok(di.db_infrastructure_generator.generate(params)?)
        }
        GenerateCommand::Devops { component_name } => {
            let params = DevopsParams { component_name };
            Ok(di.devops_generator.generate(&params)?)
        }
        GenerateCommand::FeApp { app_name } => {
            let params = FeAppParams { app_name };
            Ok(di.fe_app_generator.generate(&params, cli.argentum_path)?)
        }
        GenerateCommand::Infrastructure {
            component_name,
            entity_name,
        } => {
            let params = InfrastructureParams {
                component_name,
                entity_name,
            };
            Ok(di.infrastructure_generator.generate(&params)?)
        }
        GenerateCommand::Ui { component_name } => {
            let params = UiParams { component_name };
            Ok(di.ui_generator.generate(&params)?)
        }
    }
}
