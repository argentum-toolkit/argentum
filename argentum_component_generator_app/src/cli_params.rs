use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CliParams {
    #[command(subcommand)]
    pub command: GenerateCommand,

    #[arg(short, long)]
    pub(crate) argentum_path: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum GenerateCommand {
    DbInfrastructure(DbInfrastructureParams),
    FeApp(FeAppParams),
}

#[derive(Args, Debug, Clone)]
#[command(about = "Generate Postgres repository")]
pub(crate) struct DbInfrastructureParams {
    #[arg(short, long)]
    pub entity_name: String,

    #[arg(short, long)]
    pub component_name: String,
}

#[derive(Args, Debug, Clone)]
#[command(about = "Generate a front-end application")]
pub(crate) struct FeAppParams {
    #[arg(short, long)]
    pub app_name: String,
}
