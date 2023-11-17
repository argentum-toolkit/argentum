use crate::generator::dto::{
    DtoGenerator, OperationResponseEnumGenerator, ParamsGenerator, RequestGenerator,
    ResponseGenerator, SchemaGenerator,
};
use crate::generator::server::{
    HandlerGenerator, PreHandlerGenerator, RouterGenerator, ServerGenerator,
};
use crate::generator::{
    CargoTomlGenerator, Combiner, DiGenerator, GitIgnoreGenerator, LibGenerator, OasLoader,
    OasYamlGenerator, OpenApiGenerator,
};
use crate::template::helper::{
    camel_helper, content_type_to_type_helper, snake_helper, trim_mod_helper, upper_camel_helper,
};
use crate::template::Renderer;
use handlebars::Handlebars;
use std::sync::Arc;

pub struct DiC {
    // Public services
    pub openapi_generator: Arc<OpenApiGenerator>,
}

impl DiC {
    pub fn new(openapi_generator: Arc<OpenApiGenerator>) -> DiC {
        DiC { openapi_generator }
    }
}

pub fn di_factory() -> DiC {
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
    let renderer = Arc::new(Renderer::new(Arc::new(reg)));
    let oas_yaml_generator = Arc::new(OasYamlGenerator::new());
    let dto_generator = Arc::new(DtoGenerator::new(renderer.clone()));
    let schema_param_generator = Arc::new(ParamsGenerator::new(renderer.clone()));
    let operation_response_enum_generator =
        Arc::new(OperationResponseEnumGenerator::new(renderer.clone()));
    let response_generator = Arc::new(ResponseGenerator::new(renderer.clone()));
    let request_generator = Arc::new(RequestGenerator::new(renderer.clone()));
    let handler_generator = Arc::new(HandlerGenerator::new(renderer.clone()));
    let pre_handler_generator = Arc::new(PreHandlerGenerator::new(renderer.clone()));
    let router_generator = Arc::new(RouterGenerator::new(renderer.clone()));
    let server_generator = Arc::new(ServerGenerator::new(renderer.clone()));
    let di_generator = Arc::new(DiGenerator::new(renderer.clone()));
    let lib_generator = Arc::new(LibGenerator::new(renderer.clone()));
    let cargo_toml_generator = Arc::new(CargoTomlGenerator::new(renderer.clone()));
    let gitignore_generator = Arc::new(GitIgnoreGenerator::new(renderer.clone()));
    let schema_generator = Arc::new(SchemaGenerator::new(renderer));
    let loader = Arc::new(OasLoader::new());
    let combiner = Arc::new(Combiner::new(loader));

    let openapi_generator = Arc::new(OpenApiGenerator::new(
        combiner,
        oas_yaml_generator,
        dto_generator,
        schema_param_generator,
        operation_response_enum_generator,
        response_generator,
        request_generator,
        handler_generator,
        pre_handler_generator,
        router_generator,
        server_generator,
        di_generator,
        lib_generator,
        cargo_toml_generator,
        gitignore_generator,
        schema_generator,
    ));

    DiC::new(openapi_generator)
}
