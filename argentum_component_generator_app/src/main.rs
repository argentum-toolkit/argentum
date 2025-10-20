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
        GenerateCommand::DbInfrastructure(params) => {
            Ok(di.db_infrastructure_generator.generate(params)?)
        }
        GenerateCommand::FeApp(params) => {
            Ok(di.fe_app_generator.generate(&params, cli.argentum_path)?)
        }
    }
}
