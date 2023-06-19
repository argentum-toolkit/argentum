use crate::description::{Operation, Path};
use crate::generator::server::{HandlerGenerator, PreHandlerGenerator, RouterGenerator};
use crate::generator::{DtoGenerator, RequestGenerator, SchemaParamsGenerator};
use crate::template::Renderer;
use convert_case::{Case, Casing};
use handlebars::{handlebars_helper, Handlebars};
use hyper::Method;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

pub(crate) mod description;
pub(crate) mod generator;
pub(crate) mod template;

handlebars_helper!(snake_helper: |s: String| s.to_case(Case::Snake));

fn main() -> Result<(), Box<dyn Error>> {
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

    reg.register_helper("snake", Box::new(snake_helper));

    //services
    let renderer = Arc::new(Renderer::new(
        "../argentum_user_account/api-gen",
        Arc::new(reg),
    ));

    let dto_generator = DtoGenerator::new(renderer.clone());
    let schema_param_generator = SchemaParamsGenerator::new(renderer.clone());
    let request_generator = RequestGenerator::new(renderer.clone());
    let handler_generator = HandlerGenerator::new(renderer.clone());
    let pre_handler_generator = PreHandlerGenerator::new(renderer.clone());
    let router_generator = RouterGenerator::new(renderer);

    //data
    let operations = [
        Operation::new("AnonymousRegisters".to_string(), false, false),
        Operation::new("AnonymousRequestsRestoreToken".to_string(), true, true),
        Operation::new("AnonymousWithTokenChangesPassword".to_string(), true, true),
        Operation::new("UserLoginsWithPassword".to_string(), true, true),
        Operation::new("UserRegistersWithPassword".to_string(), true, true),
    ];

    let paths = [
        Path::new(
            "/user/anonymous-register".to_string(),
            HashMap::from([(
                Method::POST,
                Operation::new("AnonymousRegisters".to_string(), false, false),
            )]),
        ),
        Path::new(
            "/user/restore-password/token-request".to_string(),
            HashMap::from([(
                Method::POST,
                Operation::new("AnonymousRequestsRestoreToken".to_string(), true, true),
            )]),
        ),
        Path::new(
            "/user/restore-password/change-password".to_string(),
            HashMap::from([(
                Method::POST,
                Operation::new("AnonymousWithTokenChangesPassword".to_string(), true, true),
            )]),
        ),
        Path::new(
            "/user/password-login".to_string(),
            HashMap::from([(
                Method::POST,
                Operation::new("UserLoginsWithPassword".to_string(), true, true),
            )]),
        ),
        Path::new(
            "/user/password-login".to_string(),
            HashMap::from([(
                Method::POST,
                Operation::new("UserRegistersWithPassword".to_string(), true, true),
            )]),
        ),
    ];

    //generation
    dto_generator.generate()?;
    schema_param_generator.generate(&operations)?;
    request_generator.generate(&operations)?;
    handler_generator.generate(&operations)?;
    pre_handler_generator.generate(&operations)?;
    router_generator.generate(&paths)?;

    Ok(())
}
