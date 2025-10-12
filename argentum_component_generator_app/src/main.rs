use clap::Parser;

use crate::cli_params::{CliParams, GenerateCommand};
use crate::di::di_factory;
use std::error::Error;

mod cli_params;
mod di;
pub(crate) mod generator;
// pub(crate) mod template;

fn main() -> Result<(), Box<dyn Error>> {
    let cli: CliParams = CliParams::parse();

    let di = di_factory()?;

    match cli.command {
        GenerateCommand::PgRepository(params) => Ok(di.db_repository_generator.generate(params)?),
    }
}
