use clap::Parser;

use crate::cli_params::{CliParams, GenerateCommand};
use crate::di::di_factory;
use std::error::Error;

mod cli_params;
mod di;
pub(crate) mod generator;

fn main() -> Result<(), Box<dyn Error>> {
    let cli: CliParams = CliParams::parse();

    let di = di_factory()?;

    match cli.command {
        GenerateCommand::ApiDoc(params) => Ok(di.api_doc_generator.generate(&params)?),
        GenerateCommand::Business(params) => Ok(di.business_generator.generate(&params)?),
        GenerateCommand::DbInfrastructure(params) => {
            Ok(di.db_infrastructure_generator.generate(params)?)
        }
        GenerateCommand::Devops(params) => Ok(di.devops_generator.generate(&params)?),
        GenerateCommand::FeApp(params) => {
            Ok(di.fe_app_generator.generate(&params, cli.argentum_path)?)
        }
        GenerateCommand::Infrastructure(params) => {
            Ok(di.infrastructure_generator.generate(&params)?)
        }
        GenerateCommand::Component(params) => Ok(di.component_generator.generate(&params)?),
        GenerateCommand::Ui(params) => Ok(di.ui_generator.generate(&params)?),
    }
}
