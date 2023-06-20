use crate::description::{Operation, Path, Request};
use crate::generator::dto::{DtoGenerator, RequestGenerator, SchemaParamsGenerator};
use crate::generator::server::{
    HandlerGenerator, PreHandlerGenerator, RouterGenerator, ServerGenerator,
};
use crate::generator::{CargoTomlGenerator, DiGenerator, LibGenerator};
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
handlebars_helper!(trim_mod_name: |s: String| {
    s.split("::").last()
});

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

    reg.register_template_file("server/mod", "template/server/mod.hbs")
        .unwrap();

    reg.register_template_file("di", "template/di.hbs").unwrap();
    reg.register_template_file("lib", "template/lib.hbs")
        .unwrap();
    reg.register_template_file("cargo.toml", "template/cargo.toml.hbs")
        .unwrap();

    reg.register_helper("snake", Box::new(snake_helper));
    reg.register_helper("trim_mod", Box::new(trim_mod_name));

    //services
    let renderer = Arc::new(Renderer::new(
        "../argentum_user_account/rest",
        Arc::new(reg),
    ));

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

    //data
    let operations = [
        Operation::new("AnonymousRegisters".to_string(), None),
        Operation::new(
            "AnonymousRequestsRestoreToken".to_string(),
            Some(Request::new(
                "argentum_user_account_api::models::RequestRestoreTokenSchema".to_string(),
                true,
            )),
        ),
        Operation::new(
            "AnonymousWithTokenChangesPassword".to_string(),
            Some(Request::new(
                "argentum_user_account_api::models::ChangePasswordSchema".to_string(),
                true,
            )),
        ),
        Operation::new(
            "UserLoginsWithPassword".to_string(),
            Some(Request::new(
                "argentum_user_account_api::models::LoginWithPasswordSchema".to_string(),
                true,
            )),
        ),
        Operation::new(
            "UserRegistersWithPassword".to_string(),
            Some(Request::new(
                "argentum_user_account_api::models::RegistrationWithPasswordSchema".to_string(),
                true,
            )),
        ),
    ];

    let paths = [
        Path::new(
            "/user/anonymous-register".to_string(),
            HashMap::from([(
                Method::POST,
                Operation::new("AnonymousRegisters".to_string(), None),
            )]),
        ),
        Path::new(
            "/user/restore-password/token-request".to_string(),
            HashMap::from([(
                Method::POST,
                Operation::new(
                    "AnonymousRequestsRestoreToken".to_string(),
                    Some(Request::new(
                        "argentum_user_account_api::models::RequestRestoreTokenSchema".to_string(),
                        true,
                    )),
                ),
            )]),
        ),
        Path::new(
            "/user/restore-password/change-password".to_string(),
            HashMap::from([(
                Method::POST,
                Operation::new(
                    "AnonymousWithTokenChangesPassword".to_string(),
                    Some(Request::new(
                        "argentum_user_account_api::models::ChangePasswordSchema".to_string(),
                        true,
                    )),
                ),
            )]),
        ),
        Path::new(
            "/user/password-login".to_string(),
            HashMap::from([(
                Method::POST,
                Operation::new(
                    "UserLoginsWithPassword".to_string(),
                    Some(Request::new(
                        "argentum_user_account_api::models::LoginWithPasswordSchema".to_string(),
                        true,
                    )),
                ),
            )]),
        ),
        Path::new(
            "/user/register".to_string(),
            HashMap::from([(
                Method::POST,
                Operation::new(
                    "UserRegistersWithPassword".to_string(),
                    Some(Request::new(
                        "argentum_user_account_api::models::RegistrationWithPasswordSchema"
                            .to_string(),
                        true,
                    )),
                ),
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
    server_generator.generate()?;
    di_generator.generate(&operations)?;
    lib_generator.generate()?;
    cargo_toml_generator.generate()?;

    Ok(())
}
