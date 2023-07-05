use crate::generator::dto::{DtoGenerator, RequestGenerator, SchemaParamsGenerator};
use crate::generator::server::{
    HandlerGenerator, PreHandlerGenerator, RouterGenerator, ServerGenerator,
};
use crate::generator::{CargoTomlGenerator, DiGenerator, LibGenerator};
use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::SpecificationRoot;
use convert_case::{Case, Casing};
use handlebars::{handlebars_helper, Handlebars};
use std::error::Error;
use std::fs;
use std::sync::Arc;

pub(crate) mod generator;
pub(crate) mod template;

handlebars_helper!(snake_helper: |s: String| s.to_case(Case::Snake));
handlebars_helper!(trim_mod_helper: |s: String| {
    s.split("::").last()
});

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::parse();

    let mut reg = Handlebars::new();
    reg.register_template_file("dto/request.item", "template/dto/request.item.hbs")
        .unwrap();

    reg.register_template_file("dto/request.mod", "template/dto/request.mod.hbs")
        .unwrap();

    reg.register_template_file(
        "dto/schema.params.item",
        "template/dto/schema.params.item.hbs",
    )
    .unwrap();

    reg.register_template_file(
        "dto/schema.params.mod",
        "template/dto/schema.params.mod.hbs",
    )
    .unwrap();

    reg.register_template_file("dto/mod", "template/dto/mod.hbs")
        .unwrap();

    reg.register_template_file("server/handler.mod", "template/server/handler.mod.hbs")
        .unwrap();
    reg.register_template_file("server/handler.item", "template/server/handler.item.hbs")
        .unwrap();

    reg.register_template_file("server/pre_handler", "template/server/pre_handler.hbs")
        .unwrap();

    reg.register_template_file("server/router", "template/server/router.hbs")
        .unwrap();

    reg.register_template_file("server/mod", "template/server/mod.hbs")
        .unwrap();

    reg.register_template_file("di", "template/di.hbs").unwrap();
    reg.register_template_file("lib", "template/lib.hbs")
        .unwrap();
    reg.register_template_file("cargo.toml", "template/cargo.toml.hbs")
        .unwrap();

    reg.register_helper("snake", Box::new(snake_helper));
    reg.register_helper("trim_mod", Box::new(trim_mod_helper));

    //services
    let renderer = Arc::new(Renderer::new(cli.output, Arc::new(reg)));

    let dto_generator = DtoGenerator::new(renderer.clone());
    let schema_param_generator = SchemaParamsGenerator::new(renderer.clone());
    let request_generator = RequestGenerator::new(renderer.clone());
    let handler_generator = HandlerGenerator::new(renderer.clone());
    let pre_handler_generator = PreHandlerGenerator::new(renderer.clone());
    let router_generator = RouterGenerator::new(renderer.clone());
    let server_generator = ServerGenerator::new(renderer.clone());
    let di_generator = DiGenerator::new(renderer.clone());
    let lib_generator = LibGenerator::new(renderer.clone());
    let cargo_toml_generator = CargoTomlGenerator::new(renderer);

    let f = fs::File::open(cli.input).expect("LogRocket: Should have been able to read the file");

    let spec: SpecificationRoot = serde_yaml::from_reader(f).expect("Could not read values.");

    println!("{:#?}", spec);

    //generation
    dto_generator.generate()?;
    schema_param_generator.generate(&spec)?;
    request_generator.generate(&spec)?;
    handler_generator.generate(&spec)?;
    pre_handler_generator.generate(&spec)?;
    router_generator.generate(&spec)?;
    server_generator.generate()?;
    di_generator.generate(&spec)?;
    lib_generator.generate()?;
    cargo_toml_generator.generate()?;

    Ok(())
}
