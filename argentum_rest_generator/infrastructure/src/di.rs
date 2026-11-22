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
use crate::template::helper::package_name_to_container_provider_name;
use argentum_log_business::LoggerTrait;
use argentum_template_infrastructure::di::template_infrastructure_di_factory;
use std::error::Error;
use std::sync::Arc;

pub struct Di<L>
where
    L: LoggerTrait,
{
    // Public services
    pub openapi_generator: Arc<OpenApiGenerator<L>>,
}

impl<L> Di<L>
where
    L: LoggerTrait,
{
    pub fn new(openapi_generator: Arc<OpenApiGenerator<L>>) -> Self {
        Self { openapi_generator }
    }
}

pub fn di_factory<L>(logger: Arc<L>) -> Result<Di<L>, Box<dyn Error>>
where
    L: LoggerTrait + 'static,
{
    let templates = vec![
        (
            "dto/operation_response_enum.item",
            include_str!("../template/dto/operation_response_enum.item.hbs"),
        ),
        (
            "dto/operation_response_enum.mod",
            include_str!("../template/dto/operation_response_enum.mod.hbs"),
        ),
        (
            "dto/response.item",
            include_str!("../template/dto/response.item.hbs"),
        ),
        (
            "dto/response.mod",
            include_str!("../template/dto/response.mod.hbs"),
        ),
        (
            "dto/request.item",
            include_str!("../template/dto/request.item.hbs"),
        ),
        (
            "dto/request.mod",
            include_str!("../template/dto/request.mod.hbs"),
        ),
        (
            "dto/path_params.item",
            include_str!("../template/dto/path_params.item.hbs"),
        ),
        (
            "dto/path_params.mod",
            include_str!("../template/dto/path_params.mod.hbs"),
        ),
        (
            "dto/params.item",
            include_str!("../template/dto/params.item.hbs"),
        ),
        (
            "dto/params.mod",
            include_str!("../template/dto/params.mod.hbs"),
        ),
        (
            "dto/schema_object.item",
            include_str!("../template/dto/schema_object.item.hbs"),
        ),
        (
            "dto/schema_array.item",
            include_str!("../template/dto/schema_array.item.hbs"),
        ),
        (
            "dto/schema_empty.item",
            include_str!("../template/dto/schema_empty.item.hbs"),
        ),
        (
            "dto/schema_dictionary.item",
            include_str!("../template/dto/schema_dictionary.item.hbs"),
        ),
        (
            "dto/schema.mod",
            include_str!("../template/dto/schema.mod.hbs"),
        ),
        ("dto/mod", include_str!("../template/dto/mod.hbs")),
        (
            "server/handler.mod",
            include_str!("../template/server/handler.mod.hbs"),
        ),
        (
            "server/handler.item",
            include_str!("../template/server/handler.item.hbs"),
        ),
        (
            "server/pre_handler",
            include_str!("../template/server/pre_handler.hbs"),
        ),
        (
            "server/router",
            include_str!("../template/server/router.hbs"),
        ),
        ("client/mod", include_str!("../template/client/mod.hbs")),
        ("server/mod", include_str!("../template/server/mod.hbs")),
        ("di", include_str!("../template/di.hbs")),
        ("lib", include_str!("../template/lib.hbs")),
        ("cargo.toml", include_str!("../template/cargo.toml.hbs")),
        ("readme.adoc", include_str!("../template/readme.adoc.hbs")),
        (".gitignore", include_str!("../template/.gitignore.hbs")),
        ("ui/mod", include_str!("../template/ui/mod.hbs")),
        ("ui/form.item", include_str!("../template/ui/form.item.hbs")),
        ("ui/form.mod", include_str!("../template/ui/form.mod.hbs")),
        (
            "ui/callbacks.item",
            include_str!("../template/ui/callbacks.item.hbs"),
        ),
        (
            "ui/callbacks.mod",
            include_str!("../template/ui/callbacks.mod.hbs"),
        ),
        (
            "ui/input.item",
            include_str!("../template/ui/input.item.hbs"),
        ),
        (
            "ui/optional_input.item",
            include_str!("../template/ui/optional_input.item.hbs"),
        ),
        ("ui/input.mod", include_str!("../template/ui/input.mod.hbs")),
        (
            "ui/input/autocomplete_uuid",
            include_str!("../template/ui/input/autocomplete_uuid.hbs"),
        ),
        (
            "ui/input/captcha",
            include_str!("../template/ui/input/captcha.hbs"),
        ),
        (
            "ui/input/labeled_checkbox",
            include_str!("../template/ui/input/labeled_checkbox.hbs"),
        ),
        (
            "ui/input/labeled_input",
            include_str!("../template/ui/input/labeled_input.hbs"),
        ),
        (
            "ui/input/labeled_textarea",
            include_str!("../template/ui/input/labeled_textarea.hbs"),
        ),
        (
            "ui/input/object",
            include_str!("../template/ui/input/object.hbs"),
        ),
        (
            "ui/form_processor.item",
            include_str!("../template/ui/form_processor.item.hbs"),
        ),
        (
            "ui/form_processor.mod",
            include_str!("../template/ui/form_processor.mod.hbs"),
        ),
    ];

    let template_di = template_infrastructure_di_factory(
        templates,
        vec![(
            "package_name_to_container_provider_name",
            Box::new(package_name_to_container_provider_name),
        )],
    )?;

    //services
    let renderer = template_di.renderer;
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

    let schema_extractor = Arc::new(SchemaExtractor::new());
    let schema_generator = Arc::new(SchemaGenerator::new(
        renderer.clone(),
        schema_extractor.clone(),
    ));
    let loader = Arc::new(OasLoader::new(logger.clone()));
    let combiner = Arc::new(Combiner::new(logger.clone(), loader));
    let client_generator = Arc::new(ClientGenerator::new(renderer.clone()));

    let request_body_extractor = Arc::new(RequestBodyExtractor::new());

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

    Ok(Di::new(openapi_generator))
}
