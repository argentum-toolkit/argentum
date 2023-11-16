use crate::generator::dto::{
    DtoGenerator, OperationResponseEnumGenerator, ParamsGenerator, RequestGenerator,
    ResponseGenerator, SchemaGenerator,
};
use crate::generator::server::{
    HandlerGenerator, PreHandlerGenerator, RouterGenerator, ServerGenerator,
};
use crate::generator::{
    CargoTomlGenerator, Combiner, DiGenerator, GitIgnoreGenerator, LibGenerator, OasLoader,
    OasYamlGenerator,
};
use crate::template::Renderer;
use convert_case::{Case, Casing};
use handlebars::{handlebars_helper, Handlebars};
use std::error::Error;
use std::sync::Arc;

pub(crate) mod generator;
pub(crate) mod template;

handlebars_helper!(snake_helper: |s: String| s.to_case(Case::Snake));
handlebars_helper!(upper_camel_helper: |s: String| s.to_case(Case::UpperCamel));
handlebars_helper!(camel_helper: |s: String| s.to_case(Case::Camel));
handlebars_helper!(content_type_to_type_helper: |s: String| s.replace("/", "_").replace("-", "_").replace("+", "_").to_case(Case::UpperCamel));

handlebars_helper!(trim_mod_helper: |s: String| {
    s.split("::").last().unwrap_or("")
});

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long)]
    package_name: String,

    #[arg(long)]
    homepage: Option<String>,

    #[arg(short, long)]
    repository: Option<String>,

    #[arg(short, long)]
    documentation: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::parse();

    let mut reg = Handlebars::new();
    reg.register_template_file(
        "dto/operation_response_enum.item",
        "template/dto/operation_response_enum.item.hbs",
    )
    .unwrap();

    reg.register_template_file(
        "dto/operation_response_enum.mod",
        "template/dto/operation_response_enum.mod.hbs",
    )
    .unwrap();

    reg.register_template_file("dto/response.item", "template/dto/response.item.hbs")
        .unwrap();

    reg.register_template_file("dto/response.mod", "template/dto/response.mod.hbs")
        .unwrap();

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
    reg.register_helper(
        "content_type_to_type",
        Box::new(content_type_to_type_helper),
    );
    reg.register_helper("trim_mod", Box::new(trim_mod_helper));

    //services
    let renderer = Arc::new(Renderer::new(cli.output.clone(), Arc::new(reg)));

    let dto_generator = DtoGenerator::new(renderer.clone());
    let schema_param_generator = ParamsGenerator::new(renderer.clone());
    let operation_response_enum_generator = OperationResponseEnumGenerator::new(renderer.clone());
    let response_generator = ResponseGenerator::new(renderer.clone());
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
    let loader = Arc::new(OasLoader::new());
    let combiner = Combiner::new(loader);
    let oas_yaml_generator = OasYamlGenerator::new(cli.output);

    let spec = combiner.combine(cli.input);

    //generation
    oas_yaml_generator.generate(&spec)?;
    dto_generator.generate()?;
    schema_param_generator.generate(&spec)?;
    operation_response_enum_generator.generate(&spec)?;
    response_generator.generate(&spec)?;
    request_generator.generate(&spec)?;
    handler_generator.generate(&spec)?;
    pre_handler_generator.generate(&spec)?;
    router_generator.generate(&spec)?;
    server_generator.generate()?;
    di_generator.generate(&spec)?;
    lib_generator.generate()?;
    cargo_toml_generator.generate(
        &spec,
        cli.package_name,
        cli.homepage,
        cli.repository,
        cli.documentation,
    )?;
    gitignore_generator.generate()?;
    schema_generator.generate(&spec)?;

    Ok(())
}
