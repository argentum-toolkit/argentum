use crate::Renderer;
use crate::helper::{
    camel_helper, content_type_to_type_helper, eq_helper, escape_var_name_helper, lower_helper,
    snake_helper, trim_mod_helper, upper_camel_helper,
};
use handlebars::Handlebars;
use std::error::Error;
use std::sync::Arc;

pub struct TemplateInfrastructureDiC {
    // Public services
    pub renderer: Arc<Renderer>,
}

impl TemplateInfrastructureDiC {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }
}

pub fn template_infrastructure_di_factory(
    templates: Vec<(&str, &str)>,
) -> Result<TemplateInfrastructureDiC, Box<dyn Error>> {
    let mut reg = Handlebars::new();

    for (name, template) in templates.iter() {
        reg.register_template_string(name, template)?;
    }

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

    let renderer = Arc::new(Renderer::new(Arc::new(reg)));

    Ok(TemplateInfrastructureDiC::new(renderer))
}
