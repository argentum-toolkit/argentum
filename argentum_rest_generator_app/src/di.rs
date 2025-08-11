use crate::extractor::{RequestBodyExtractor, SchemaExtractor};
use crate::generator::client::ClientGenerator;
use crate::generator::dto::{
    DtoGenerator, OperationResponseEnumGenerator, ParamsGenerator, PathParamsGenerator,
    RequestGenerator, ResponseGenerator, SchemaGenerator,
};
use crate::generator::path_param::regex::{IntegerFactory, RegexFactory, StringFactory};
use crate::generator::server::{
    HandlerGenerator, PreHandlerGenerator, RouterGenerator, ServerGenerator,
};
use crate::generator::ui::{
    CallbacksGenerator, FormGenerator, FormProcessorGenerator, InputGenerator, UiGenerator,
};
use crate::generator::{
    CargoTomlGenerator, Combiner, DiGenerator, GitIgnoreGenerator, LibGenerator, OasLoader,
    OasYamlGenerator, OpenApiGenerator, ReadmeAdocGenerator,
};
use crate::template::Renderer;
use crate::template::helper::{
    camel_helper, content_type_to_type_helper, eq_helper, escape_var_name_helper, lower_helper,
    snake_helper, trim_mod_helper, upper_camel_helper,
};
use argentum_log_business::{DefaultLogger, Level, LoggerTrait};
use argentum_log_infrastructure::stdout::PrettyWriter;
use handlebars::Handlebars;
use std::error::Error;
use std::sync::Arc;

pub struct DiC<L>
where
    L: LoggerTrait,
{
    // Public services
    pub openapi_generator: Arc<OpenApiGenerator<L>>,
}

impl<L> DiC<L>
where
    L: LoggerTrait,
{
    pub fn new(openapi_generator: Arc<OpenApiGenerator<L>>) -> Self {
        Self { openapi_generator }
    }
}

pub fn di_factory() -> Result<DiC<DefaultLogger<PrettyWriter>>, Box<dyn Error>> {
    let mut reg = Handlebars::new();
    reg.register_template_string(
        "dto/operation_response_enum.item",
        include_str!("../template/dto/operation_response_enum.item.hbs"),
    )?;

    reg.register_template_string(
        "dto/operation_response_enum.mod",
        include_str!("../template/dto/operation_response_enum.mod.hbs"),
    )?;

    reg.register_template_string(
        "dto/response.item",
        include_str!("../template/dto/response.item.hbs"),
    )?;

    reg.register_template_string(
        "dto/response.mod",
        include_str!("../template/dto/response.mod.hbs"),
    )?;

    reg.register_template_string(
        "dto/request.item",
        include_str!("../template/dto/request.item.hbs"),
    )?;

    reg.register_template_string(
        "dto/request.mod",
        include_str!("../template/dto/request.mod.hbs"),
    )?;

    reg.register_template_string(
        "dto/path_params.item",
        include_str!("../template/dto/path_params.item.hbs"),
    )?;

    reg.register_template_string(
        "dto/path_params.mod",
        include_str!("../template/dto/path_params.mod.hbs"),
    )?;

    reg.register_template_string(
        "dto/params.item",
        include_str!("../template/dto/params.item.hbs"),
    )?;

    reg.register_template_string(
        "dto/params.mod",
        include_str!("../template/dto/params.mod.hbs"),
    )?;

    reg.register_template_string(
        "dto/schema_object.item",
        include_str!("../template/dto/schema_object.item.hbs"),
    )?;

    reg.register_template_string(
        "dto/schema_array.item",
        include_str!("../template/dto/schema_array.item.hbs"),
    )?;

    reg.register_template_string(
        "dto/schema_dictionary.item",
        include_str!("../template/dto/schema_dictionary.item.hbs"),
    )?;

    reg.register_template_string(
        "dto/schema.mod",
        include_str!("../template/dto/schema.mod.hbs"),
    )?;

    reg.register_template_string("dto/mod", include_str!("../template/dto/mod.hbs"))?;

    reg.register_template_string(
        "server/handler.mod",
        include_str!("../template/server/handler.mod.hbs"),
    )?;

    reg.register_template_string(
        "server/handler.item",
        include_str!("../template/server/handler.item.hbs"),
    )?;

    reg.register_template_string(
        "server/pre_handler",
        include_str!("../template/server/pre_handler.hbs"),
    )?;

    reg.register_template_string(
        "server/router",
        include_str!("../template/server/router.hbs"),
    )?;

    reg.register_template_string("client/mod", include_str!("../template/client/mod.hbs"))?;
    reg.register_template_string("server/mod", include_str!("../template/server/mod.hbs"))?;
    reg.register_template_string("di", include_str!("../template/di.hbs"))?;
    reg.register_template_string("lib", include_str!("../template/lib.hbs"))?;
    reg.register_template_string("cargo.toml", include_str!("../template/cargo.toml.hbs"))?;
    reg.register_template_string("readme.adoc", include_str!("../template/readme.adoc.hbs"))?;
    reg.register_template_string(".gitignore", include_str!("../template/.gitignore.hbs"))?;

    reg.register_template_string("ui/mod", include_str!("../template/ui/mod.hbs"))?;
    reg.register_template_string("ui/form.item", include_str!("../template/ui/form.item.hbs"))?;
    reg.register_template_string("ui/form.mod", include_str!("../template/ui/form.mod.hbs"))?;

    reg.register_template_string(
        "ui/callbacks.item",
        include_str!("../template/ui/callbacks.item.hbs"),
    )?;

    reg.register_template_string(
        "ui/callbacks.mod",
        include_str!("../template/ui/callbacks.mod.hbs"),
    )?;

    reg.register_template_string(
        "ui/input.item",
        include_str!("../template/ui/input.item.hbs"),
    )?;

    reg.register_template_string("ui/input.mod", include_str!("../template/ui/input.mod.hbs"))?;

    reg.register_template_string(
        "ui/input/labeled_checkbox",
        include_str!("../template/ui/input/labeled_checkbox.hbs"),
    )?;

    reg.register_template_string(
        "ui/input/labeled_input",
        include_str!("../template/ui/input/labeled_input.hbs"),
    )?;

    reg.register_template_string(
        "ui/input/object",
        include_str!("../template/ui/input/object.hbs"),
    )?;

    reg.register_template_string(
        "ui/form_processor.item",
        include_str!("../template/ui/form_processor.item.hbs"),
    )?;

    reg.register_template_string(
        "ui/form_processor.mod",
        include_str!("../template/ui/form_processor.mod.hbs"),
    )?;

    reg.register_helper("snake", Box::new(snake_helper));
    reg.register_helper("camel", Box::new(camel_helper));
    reg.register_helper("upper_camel", Box::new(upper_camel_helper));
    reg.register_helper("lower", Box::new(lower_helper));
    reg.register_helper("eq", Box::new(eq_helper));

    reg.register_helper(
        "content_type_to_type",
        Box::new(content_type_to_type_helper),
    );

    reg.register_helper("trim_mod", Box::new(trim_mod_helper));
    reg.register_helper("escape_var_name", Box::new(escape_var_name_helper));

    //services
    let log_writer = Arc::new(PrettyWriter::new());
    let logger = Arc::new(DefaultLogger::new(Level::Trace, log_writer));
    let renderer = Arc::new(Renderer::new(Arc::new(reg)));
    let oas_yaml_generator = Arc::new(OasYamlGenerator::new());
    let dto_generator = Arc::new(DtoGenerator::new(renderer.clone()));
    let path_param_generator = Arc::new(PathParamsGenerator::new(renderer.clone()));
    let schema_param_generator = Arc::new(ParamsGenerator::new(renderer.clone()));
    let operation_response_enum_generator =
        Arc::new(OperationResponseEnumGenerator::new(renderer.clone()));
    let response_generator = Arc::new(ResponseGenerator::new(renderer.clone()));
    let request_generator = Arc::new(RequestGenerator::new(renderer.clone()));
    let handler_generator = Arc::new(HandlerGenerator::new(renderer.clone()));
    let pre_handler_generator = Arc::new(PreHandlerGenerator::new(renderer.clone()));
    let regex_factory = Arc::new(RegexFactory::new(
        Arc::new(StringFactory::new()),
        Arc::new(IntegerFactory::new()),
    ));
    let router_generator = Arc::new(RouterGenerator::new(renderer.clone(), regex_factory));
    let server_generator = Arc::new(ServerGenerator::new(renderer.clone()));
    let di_generator = Arc::new(DiGenerator::new(renderer.clone()));
    let lib_generator = Arc::new(LibGenerator::new(renderer.clone()));
    let cargo_toml_generator = Arc::new(CargoTomlGenerator::new(renderer.clone()));
    let readme_adoc_generator = Arc::new(ReadmeAdocGenerator::new(renderer.clone()));
    let gitignore_generator = Arc::new(GitIgnoreGenerator::new(renderer.clone()));
    let schema_generator = Arc::new(SchemaGenerator::new(renderer.clone()));
    let loader = Arc::new(OasLoader::new(logger.clone()));
    let combiner = Arc::new(Combiner::new(logger.clone(), loader));
    let client_generator = Arc::new(ClientGenerator::new(renderer.clone()));

    let request_body_extractor = Arc::new(RequestBodyExtractor::new());
    let schema_extractor = Arc::new(SchemaExtractor::new());

    let form_generator = Arc::new(FormGenerator::new(
        renderer.clone(),
        request_body_extractor.clone(),
        schema_extractor.clone(),
    ));

    let form_data_generator = Arc::new(CallbacksGenerator::new(renderer.clone()));

    let input_generator = Arc::new(InputGenerator::new(
        renderer.clone(),
        request_body_extractor.clone(),
        schema_extractor.clone(),
    ));

    let web_boilerplate_generator = Arc::new(FormProcessorGenerator::new(
        renderer.clone(),
        request_body_extractor,
        schema_extractor,
    ));

    let ui_generator = Arc::new(UiGenerator::new(
        renderer,
        form_generator,
        form_data_generator,
        input_generator,
        web_boilerplate_generator,
    ));

    let openapi_generator = Arc::new(OpenApiGenerator::new(
        logger.clone(),
        combiner,
        oas_yaml_generator,
        dto_generator,
        path_param_generator,
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
        readme_adoc_generator,
        gitignore_generator,
        schema_generator,
        client_generator,
        ui_generator,
    ));

    Ok(DiC::<DefaultLogger<PrettyWriter>>::new(openapi_generator))
}
