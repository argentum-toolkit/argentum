use convert_case::{Case, Casing};
use handlebars::handlebars_helper;

handlebars_helper!(snake_helper: |s: Option<String>| s.unwrap_or("".into()).to_case(Case::Snake));
handlebars_helper!(upper_camel_helper: |s: Option<String>| s.unwrap_or("".into()).to_case(Case::UpperCamel));
handlebars_helper!(camel_helper: |s: Option<String>| s.unwrap_or("".into()).to_case(Case::Camel));
handlebars_helper!(lower_helper: |s: Option<String>| s.unwrap_or("".into()).to_case(Case::Lower));
handlebars_helper!(content_type_to_type_helper: |s: Option<String>| s.unwrap_or("".into()).replace(['/', '-', '+'], "_").to_case(Case::UpperCamel));

handlebars_helper!(eq_helper: |a: String, b: String| {
    a == b
});

handlebars_helper!(trim_mod_helper: |s: String| {
    s.split("::").last().unwrap_or("")
});

handlebars_helper!(escape_var_name_helper: |s: String| {
    match s.as_str() {
        "type" => "r#type".to_string(),
        _ => s,
    }
});
