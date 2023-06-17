use crate::description::Operation;
use crate::generator::{RequestGenerator, SchemaParamsGenerator};
use crate::template::Renderer;
use convert_case::{Case, Casing};
use handlebars::{handlebars_helper, Handlebars};
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

    reg.register_helper("snake", Box::new(snake_helper));

    //services
    let renderer = Arc::new(Renderer::new(
        "../argentum_user_account/api-gen",
        Arc::new(reg),
    ));

    let schema_param_generator = SchemaParamsGenerator::new(renderer.clone());
    let request_generator = RequestGenerator::new(renderer);

    //data
    let operations = [
        Operation::new("AnonymousRequestsRestoreToken".to_string()),
        Operation::new("AnonymousWithTokenChangesPassword".to_string()),
        Operation::new("UserLoginsWithPassword".to_string()),
        Operation::new("UserRegistersWithPassword".to_string()),
    ];

    //generation
    schema_param_generator.generate(&operations)?;
    request_generator.generate(&operations)?;

    Ok(())
}
