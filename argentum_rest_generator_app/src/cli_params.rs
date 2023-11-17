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
}
