use std::collections::BTreeMap;
use crate::generator::dto::{DtoGenerator, ParamsGenerator, RequestGenerator, SchemaGenerator};
use crate::generator::server::{
    HandlerGenerator, PreHandlerGenerator, RouterGenerator, ServerGenerator,
};
use crate::generator::{CargoTomlGenerator, DiGenerator, GitIgnoreGenerator, LibGenerator};
use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{Components, RefOrObject, Schema, SchemaType, SpecificationRoot};
use convert_case::{Case, Casing};
use handlebars::{handlebars_helper, Handlebars};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

pub(crate) mod generator;
pub(crate) mod template;

handlebars_helper!(snake_helper: |s: String| s.to_case(Case::Snake));
handlebars_helper!(upper_camel_helper: |s: String| s.to_case(Case::UpperCamel));
handlebars_helper!(camel_helper: |s: String| s.to_case(Case::Camel));

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

    reg.register_template_file("dto/params.item", "template/dto/params.item.hbs")
        .unwrap();

    reg.register_template_file("dto/params.mod", "template/dto/params.mod.hbs")
        .unwrap();

    reg.register_template_file("dto/schema.item", "template/dto/schema.item.hbs")
        .unwrap();

    reg.register_template_file("dto/schema.mod", "template/dto/schema.mod.hbs")
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
    reg.register_template_file(".gitignore", "template/.gitignore.hbs")
        .unwrap();

    reg.register_helper("snake", Box::new(snake_helper));
    reg.register_helper("camel", Box::new(camel_helper));
    reg.register_helper("upper_camel", Box::new(upper_camel_helper));
    reg.register_helper("trim_mod", Box::new(trim_mod_helper));

    //services
    let renderer = Arc::new(Renderer::new(cli.output, Arc::new(reg)));

    let dto_generator = DtoGenerator::new(renderer.clone());
    let schema_param_generator = ParamsGenerator::new(renderer.clone());
    let request_generator = RequestGenerator::new(renderer.clone());
    let handler_generator = HandlerGenerator::new(renderer.clone());
    let pre_handler_generator = PreHandlerGenerator::new(renderer.clone());
    let router_generator = RouterGenerator::new(renderer.clone());
    let server_generator = ServerGenerator::new(renderer.clone());
    let di_generator = DiGenerator::new(renderer.clone());
    let lib_generator = LibGenerator::new(renderer.clone());
    let cargo_toml_generator = CargoTomlGenerator::new(renderer.clone());
    let gitignore_generator = GitIgnoreGenerator::new(renderer.clone());
    let schema_generator = SchemaGenerator::new(renderer);



    let (mut spec, full_path) = load(cli.input);

    //todo: combine

    println!("{:#?}", res_spec);

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
    gitignore_generator.generate()?;
    schema_generator.generate(&spec)?;

    Ok(())
}

/// String: full path of current file with OpenAPI specification
fn load(file_path: String) -> (SpecificationRoot, PathBuf) {

    let path = PathBuf::from(file_path);

    let f = fs::File::open(path.clone()).expect("LogRocket: Should have been able to read the file");

    let spec: SpecificationRoot = serde_yaml::from_reader(f).expect("Could not read values.");

    (spec, path)
}