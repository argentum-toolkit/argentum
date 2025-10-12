use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CliParams {
    #[command(subcommand)]
    pub command: GenerateCommand,
}

#[derive(Subcommand, Debug)]
pub enum GenerateCommand {
    DbInfrastructure(DbInfrastructureParams),
}

#[derive(Args, Debug, Clone)]
#[command(about = "Generate Postgres repository")]
pub(crate) struct DbInfrastructureParams {
    #[arg(short, long)]
    pub entity_name: String,

    #[arg(short, long)]
    pub component_name: String,
}
