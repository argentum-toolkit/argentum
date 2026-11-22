use argentum_log_business::{DefaultLogger, Level};
use argentum_log_infrastructure::stdout::PrettyWriter;
use argentum_rest_generator_infrastructure::di::di_factory;
use argentum_rest_generator_infrastructure::generator_params::GeneratorParams;
use clap::Parser;
use std::error::Error;
use std::sync::Arc;

mod cli_params;

use crate::cli_params::CliParams;

fn main() -> Result<(), Box<dyn Error>> {
    let cli: CliParams = CliParams::parse();

    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));

    let di = di_factory(logger)?;

    let params = GeneratorParams::from(cli);

    Ok(di.openapi_generator.generate(params)?)
}
