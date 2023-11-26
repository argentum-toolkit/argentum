use convert_case::{Case, Casing};
use handlebars::handlebars_helper;

handlebars_helper!(snake_helper: |s: String| s.to_case(Case::Snake));
handlebars_helper!(upper_camel_helper: |s: String| s.to_case(Case::UpperCamel));
handlebars_helper!(camel_helper: |s: String| s.to_case(Case::Camel));
handlebars_helper!(content_type_to_type_helper: |s: String| s.replace("/", "_").replace("-", "_").replace("+", "_").to_case(Case::UpperCamel));

handlebars_helper!(eq_helper: |a: String, b: String| {
    a == b
});

handlebars_helper!(trim_mod_helper: |s: String| {
    s.split("::").last().unwrap_or("")
});
