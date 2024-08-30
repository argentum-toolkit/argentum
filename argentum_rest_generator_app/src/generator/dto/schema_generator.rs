use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{
    RefOrObject, Schema, SchemaFormat, SchemaType, SpecificationRoot, StandardFormat,
};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::collections::BTreeMap;
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ArrayData<'a> {
    dependencies: Vec<String>,
    name: &'a str,
    items_type: ItemType,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ItemType {
    data_type: String,
    raw_type: String,
    is_ref: bool,
}

pub(crate) struct SchemaGenerator {
    renderer: Arc<Renderer>,
}

const MOD_PATH: &str = "/src/dto/schema/mod.rs";
const MOD_TEMPLATE: &str = "dto/schema.mod";
const OBJECT_ITEM_TEMPLATE: &str = "dto/schema_object.item";
const ARRAY_ITEM_TEMPLATE: &str = "dto/schema_array.item";

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

        match schema.schema_type {
            Some(SchemaType::Object) => {
                let props = schema.properties.clone().unwrap_or_default();
                self.generate_object_item(
                    base_output_path,
                    name,
                    props,
                    schema.required.clone(),
                    file_path,
                )?;
            }
            Some(SchemaType::Array) => match &*schema.items {
                Some(items) => {
                    self.generate_array_item(base_output_path, name, items.clone(), file_path)?
                }
                None => {}
            },
            Some(_) => {
                //TODO: implement for other types
                //TODO: log
            }
            None => {
                if let Some(props) = schema.properties.clone() {
                    self.generate_object_item(
                        base_output_path,
                        name,
                        props,
                        schema.required.clone(),
                        file_path,
                    )?;
                } else if let Some(items) = &*schema.items {
                    self.generate_array_item(base_output_path, name, items.clone(), file_path)?;
                } else {
                    //TODO: implement for other types
                }
            }
        };

        Ok(())
    }

    ///
    /// Returns (final_data_type, raw_type, is_ref)
    fn schema_to_rs(
        &self,
        property: RefOrObject<Schema>,
        dependencies: &mut Vec<String>,
    ) -> (String, String, bool) {
        //todo: check $ref
        match property {
            RefOrObject::Object(schema) => match schema.schema_type {
                None => ("()".to_string(), "Option<()>".to_string(), false),
                Some(SchemaType::Boolean) => {
                    ("bool".to_string(), "Option<bool>".to_string(), false)
                }
                Some(SchemaType::Integer) => match schema.format {
                    None => ("i64".to_string(), "Option<i64>".to_string(), false),
                    Some(SchemaFormat::Standard(StandardFormat::Int32)) => {
                        ("i32".to_string(), "Option<i32>".to_string(), false)
                    }
                    Some(SchemaFormat::Standard(StandardFormat::Int64)) => {
                        ("i64".to_string(), "Option<i64>".to_string(), false)
                    }
                    Some(SchemaFormat::Standard(StandardFormat::UInt32)) => {
                        ("u32".to_string(), "Option<u32>".to_string(), false)
                    }
                    Some(SchemaFormat::Standard(StandardFormat::UInt64)) => {
                        ("u64".to_string(), "Option<u64>".to_string(), false)
                    }
                    Some(_) => ("i64".to_string(), "Option<i64>".to_string(), false),
                },
                Some(SchemaType::Number) => match schema.format {
                    None => ("f64".to_string(), "Option<f64>".to_string(), false),
                    Some(SchemaFormat::Standard(StandardFormat::Float)) => {
                        ("f32".to_string(), "Option<f32>".to_string(), false)
                    }
                    Some(SchemaFormat::Standard(StandardFormat::Double)) => {
                        ("f64".to_string(), "Option<f64>".to_string(), false)
                    }
                    Some(_) => ("f64".to_string(), "Option<f64>".to_string(), false),
                },
                Some(SchemaType::String) => match schema.format {
                    None => ("String".to_string(), "Option<String>".to_string(), false),
                    Some(SchemaFormat::Standard(StandardFormat::Date)) => {
                        dependencies.push("chrono::NaiveDate".to_string());
                        (
                            "NaiveDate".to_string(),
                            "Option<NaiveDate>".to_string(),
                            false,
                        )
                    }
                    Some(SchemaFormat::Standard(StandardFormat::DateTime)) => {
                        dependencies.push("chrono::{DateTime, Utc}".to_string());
                        (
                            "DateTime<Utc>".to_string(),
                            "Option<DateTime<Utc>>".to_string(),
                            false,
                        )
                    }
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
        }
    }

    fn generate_object_item(
        &self,
        base_output_path: &str,
        name: &String,
        schema_properties: BTreeMap<String, RefOrObject<Schema>>,
        required_fields: Option<Vec<String>>,
        file_path: String,
    ) -> Result<(), Box<dyn Error>> {
        let mut properties: Vec<Prop> = vec![];
        let mut dependencies: Vec<String> = vec![];

        for (name, property) in schema_properties {
            let (mut data_type, raw_type, is_ref) = self.schema_to_rs(property, &mut dependencies);

            let req = &required_fields.clone().unwrap_or_default();

            let required: bool;

            if req.contains(&name) {
                required = true;
            } else {
                data_type = format!("Option<{}>", data_type);
                required = false;
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

        let data = Data {
            dependencies,
            name,
            properties,
        };

        self.renderer.render(
            base_output_path,
            OBJECT_ITEM_TEMPLATE,
            &data,
            file_path.as_str(),
        )?;

        Ok(())
    }

    fn generate_array_item(
        &self,
        base_output_path: &str,
        name: &String,
        items_type: RefOrObject<Schema>,
        file_path: String,
    ) -> Result<(), Box<dyn Error>> {
        let mut dependencies: Vec<String> = vec![];

        let (data_type, raw_type, is_ref) = self.schema_to_rs(items_type, &mut dependencies);

        let data = ArrayData {
            dependencies,
            name,
            items_type: ItemType {
                data_type,
                raw_type,
                is_ref,
            },
        };

        self.renderer.render(
            base_output_path,
            ARRAY_ITEM_TEMPLATE,
            &data,
            file_path.as_str(),
        )?;

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
