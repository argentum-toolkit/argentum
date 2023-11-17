use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{
    RefOrObject, Schema, SchemaFormat, SchemaType, SpecificationRoot, StandardFormat,
};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data<'a> {
    dependencies: Vec<String>,
    name: &'a str,
    properties: Vec<Prop>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Prop {
    name: String,
    rename: String,
    data_type: String,
    raw_type: String,
    required: bool,
    is_ref: bool,
}

pub(crate) struct SchemaGenerator {
    renderer: Arc<Renderer>,
}

const MOD_PATH: &str = "/src/dto/schema/mod.rs";
const MOD_TEMPLATE: &str = "dto/schema.mod";
const ITEM_TEMPLATE: &str = "dto/schema.item";

impl SchemaGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    fn generate_item(
        &self,
        base_output_path: &str,
        name: &String,
        schema: &Schema,
    ) -> Result<(), Box<dyn Error>> {
        let file_path = format!("/src/dto/schema/{}.rs", name.to_case(Case::Snake));

        let mut properties: Vec<Prop> = vec![];
        let mut dependencies: Vec<String> = vec![];

        if schema.properties.is_some() {
            for (name, property) in schema.properties.clone().unwrap() {
                //todo: check $ref
                let (mut data_type, raw_type, is_ref) = match property {
                    RefOrObject::Object(schema) => match schema.schema_type {
                        None => ("()".to_string(), "Option<()>".to_string(), false),
                        Some(SchemaType::String) => match schema.format {
                            None => ("String".to_string(), "Option<String>".to_string(), false),
                            Some(SchemaFormat::Standard(StandardFormat::Uuid)) => (
                                "uuid::Uuid".to_string(),
                                "Option<uuid::Uuid>".to_string(),
                                false,
                            ),
                            Some(_) => ("String".to_string(), "Option<String>".to_string(), false),
                        },
                        Some(_) => ("String".to_string(), "Option<String>".to_string(), false),
                    },
                    RefOrObject::Ref(r) => {
                        let type_name = r
                            .reference
                            .clone()
                            .split('/')
                            .last()
                            .unwrap_or_else(|| {
                                panic!(
                                    "Wrong schema href {}. Expected: `#/components/schemas/{{name}}`",
                                    r.reference
                                )
                            })
                            .to_string();

                        dependencies.push(format!("crate::dto::schema::{}", type_name));
                        dependencies.push(format!("crate::dto::schema::{}Raw", type_name));

                        (type_name.clone(), format!("Option<{}Raw>", type_name), true)
                    }
                };

                let req = &schema.required;

                let required: bool;

                if req.is_none() || !req.clone().unwrap().contains(&name) {
                    data_type = format!("Option<{}>", data_type);
                    required = false;
                } else {
                    required = true;
                }

                properties.push(Prop {
                    name: name.clone().to_case(Case::Snake),
                    rename: name,
                    data_type,
                    raw_type,
                    required,
                    is_ref,
                })
            }
        }

        let data = Data {
            dependencies,
            name,
            properties,
        };

        self.renderer
            .render(base_output_path, ITEM_TEMPLATE, &data, file_path.as_str())?;

        Ok(())
    }

    pub fn generate(
        &self,
        base_output_path: &str,
        spec: &SpecificationRoot,
    ) -> Result<(), Box<dyn Error>> {
        self.renderer
            .render(base_output_path, MOD_TEMPLATE, spec, MOD_PATH)?;

        for (name, schema) in &spec.components.schemas {
            self.generate_item(base_output_path, name, schema)?;
        }

        Ok(())
    }
}
