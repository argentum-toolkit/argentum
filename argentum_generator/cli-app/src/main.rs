use argentum_generator_infrastructure::di::di_factory;
use argentum_generator_infrastructure::params::{
    ApiDocParams, BusinessParams, ComponentParams, DbInfrastructureParams, DevopsParams,
    FeAppParams, InfrastructureParams, RestParams, UiParams,
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
        GenerateCommand::ApiDoc {
            project_prefix,
            component_name,
        } => {
            let params = ApiDocParams {
                project_prefix,
                component_name,
            };
            Ok(di.api_doc_generator.generate(&params)?)
        }
        GenerateCommand::Business {
            project_prefix,
            component_name,
            entity_name,
        } => {
            let params = BusinessParams {
                project_prefix,
                component_name,
                entity_name,
            };
            Ok(di.business_generator.generate(&params)?)
        }
        GenerateCommand::Component {
            project_prefix,
            component_name,
            entity_name,
        } => {
            let params = ComponentParams {
                project_prefix,
                component_name,
                entity_name,
            };
            Ok(di.component_generator.generate(&params)?)
        }
        GenerateCommand::DbInfrastructure {
            component_name,
            entity_name,
            project_prefix,
        } => {
            let params = DbInfrastructureParams {
                entity_name,
                project_prefix,
                component_name,
            };
            Ok(di.db_infrastructure_generator.generate(params)?)
        }
        GenerateCommand::Devops {
            project_prefix,
            component_name,
        } => {
            let params = DevopsParams {
                project_prefix,
                component_name,
            };
            Ok(di.devops_generator.generate(&params)?)
        }
        GenerateCommand::FeApp { app_name } => {
            let params = FeAppParams { app_name };
            Ok(di.fe_app_generator.generate(&params, cli.argentum_path)?)
        }
        GenerateCommand::Infrastructure {
            project_prefix,
            component_name,
            entity_name,
        } => {
            let params = InfrastructureParams {
                project_prefix,
                component_name,
                entity_name,
            };
            Ok(di.infrastructure_generator.generate(&params)?)
        }
        GenerateCommand::Rest {
            project_prefix,
            component_name,
        } => {
            let params = RestParams {
                project_prefix,
                component_name,
            };
            Ok(di.rest_generator.generate(&params)?)
        }
        GenerateCommand::Ui {
            project_prefix,
            component_name,
        } => {
            let params = UiParams {
                project_prefix,
                component_name,
            };
            Ok(di.ui_generator.generate(&params)?)
        }
    }
}
