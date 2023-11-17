use clap::Parser;

use crate::cli_params::CliParams;
use crate::di::di_factory;
use std::error::Error;

mod cli_params;
mod di;
pub(crate) mod generator;
pub(crate) mod template;

fn main() -> Result<(), Box<dyn Error>> {
    let cli: CliParams = CliParams::parse();

    let di = di_factory();

    return di.openapi_generator.generate(cli);
}
