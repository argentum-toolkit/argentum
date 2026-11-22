use argentum_rest_generator_infrastructure::generator_params::GeneratorParams;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliParams {
    #[arg(short, long)]
    pub(crate) input: String,

    #[arg(short, long)]
    pub(crate) output: String,

    #[arg(short, long)]
    pub(crate) package_name: String,

    #[arg(long)]
    pub(crate) homepage: Option<String>,

    #[arg(short, long)]
    pub(crate) repository: Option<String>,

    #[arg(short, long)]
    pub(crate) documentation: Option<String>,

    #[arg(short, long)]
    pub(crate) argentum_path: Option<String>,
}

impl From<CliParams> for GeneratorParams {
    fn from(cli: CliParams) -> Self {
        Self {
            input: cli.input,
            output: cli.output,
            package_name: cli.package_name,
            homepage: cli.homepage,
            repository: cli.repository,
            documentation: cli.documentation,
            argentum_path: cli.argentum_path,
        }
    }
}
