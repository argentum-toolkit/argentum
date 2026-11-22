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
    /// Generate an api-doc folder for a component
    ApiDoc {
        #[arg(short, long)]
        project_prefix: String,

        #[arg(short, long)]
        component_name: String,
    },
    /// Generate a business package skeleton
    Business {
        #[arg(short, long)]
        project_prefix: String,

        #[arg(short, long)]
        component_name: String,

        #[arg(short, long)]
        entity_name: String,
    },
    /// Generate a whole component skeleton (business, api-doc, devops, infrastructure, ui)
    Component {
        #[arg(short, long)]
        project_prefix: String,

        #[arg(short, long)]
        component_name: String,

        /// Defaults to the PascalCase form of the component name
        #[arg(short, long)]
        entity_name: Option<String>,
    },
    /// Generate Postgres repository
    DbInfrastructure {
        #[arg(short, long)]
        entity_name: String,

        #[arg(short, long)]
        project_prefix: String,

        #[arg(short, long)]
        component_name: String,
    },
    /// Generate a devops folder for a component
    Devops {
        #[arg(short, long)]
        project_prefix: String,

        #[arg(short, long)]
        component_name: String,
    },
    /// Generate a front-end application
    FeApp {
        #[arg(short, long)]
        app_name: String,
    },
    /// Generate an infrastructure package skeleton
    Infrastructure {
        #[arg(short, long)]
        project_prefix: String,

        #[arg(short, long)]
        component_name: String,

        #[arg(short, long)]
        entity_name: String,
    },
    /// Generate the rest package for a component from its api-doc OpenAPI spec
    Rest {
        #[arg(short, long)]
        project_prefix: String,

        #[arg(short, long)]
        component_name: String,
    },
    /// Generate a UI package skeleton for a component
    Ui {
        #[arg(short, long)]
        project_prefix: String,

        #[arg(short, long)]
        component_name: String,
    },
}
