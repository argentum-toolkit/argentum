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
    ApiDoc(ApiDocParams),
    Business(BusinessParams),
    Component(ComponentParams),
    DbInfrastructure(DbInfrastructureParams),
    Devops(DevopsParams),
    FeApp(FeAppParams),
    Infrastructure(InfrastructureParams),
    Ui(UiParams),
}

#[derive(Args, Debug, Clone)]
#[command(about = "Generate an api-doc folder for a component")]
pub(crate) struct ApiDocParams {
    #[arg(short, long)]
    pub component_name: String,
}

#[derive(Args, Debug, Clone)]
#[command(about = "Generate a business package skeleton")]
pub(crate) struct BusinessParams {
    #[arg(short, long)]
    pub component_name: String,

    #[arg(short, long)]
    pub entity_name: String,
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
#[command(about = "Generate an infrastructure package skeleton")]
pub(crate) struct InfrastructureParams {
    #[arg(short, long)]
    pub component_name: String,

    #[arg(short, long)]
    pub entity_name: String,
}

#[derive(Args, Debug, Clone)]
#[command(
    about = "Generate a whole component skeleton (business, api-doc, devops, infrastructure, ui)"
)]
pub(crate) struct ComponentParams {
    #[arg(short, long)]
    pub component_name: String,

    /// Defaults to the PascalCase form of the component name
    #[arg(short, long)]
    pub entity_name: Option<String>,
}

#[derive(Args, Debug, Clone)]
#[command(about = "Generate a devops folder for a component")]
pub(crate) struct DevopsParams {
    #[arg(short, long)]
    pub component_name: String,
}

#[derive(Args, Debug, Clone)]
#[command(about = "Generate a front-end application")]
pub(crate) struct FeAppParams {
    #[arg(short, long)]
    pub app_name: String,
}

#[derive(Args, Debug, Clone)]
#[command(about = "Generate a UI package skeleton for a component")]
pub(crate) struct UiParams {
    #[arg(short, long)]
    pub component_name: String,
}
