use crate::extractor::SchemaExtractor;
use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{
    RefOrObject, Schema, SchemaFormat, SchemaType, SpecificationRoot, StandardFormat,
};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::collections::BTreeMap;
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
    clean_type: String,
    data_type: String,
    raw_type: String,
    required: bool,
    is_ref: bool,
    weight: i16,
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
    schema_extractor: Arc<SchemaExtractor>,
}

const MOD_PATH: &str = "/src/dto/schema/mod.rs";
const MOD_TEMPLATE: &str = "dto/schema.mod";
const OBJECT_ITEM_TEMPLATE: &str = "dto/schema_object.item";
const ARRAY_ITEM_TEMPLATE: &str = "dto/schema_array.item";
const DICTIONARY_ITEM_TEMPLATE: &str = "dto/schema_dictionary.item";

impl SchemaGenerator {
    pub fn new(renderer: Arc<Renderer>, schema_extractor: Arc<SchemaExtractor>) -> Self {
        Self {
            renderer,
            schema_extractor,
        }
    }

    fn generate_item(
        &self,
        base_output_path: &str,
        name: &String,
        schema: &Schema,
        spec: &SpecificationRoot,
    ) -> Result<(), String> {
        let file_path = format!("/src/dto/schema/{}.rs", name.to_case(Case::Snake));

        match schema.schema_type {
            Some(SchemaType::Object) => match *schema.additional_properties.clone() {
                Some(additional) => {
                    self.generate_dictionary_item(
                        base_output_path,
                        name,
                        additional.clone(),
                        file_path,
                    )?;
                }
                None => {
                    let props = schema.properties.clone().unwrap_or_default();
                    self.generate_object_item(
                        base_output_path,
                        name,
                        props,
                        schema.required.clone(),
                        file_path,
                        spec,
                    )?;
                }
            },
            Some(SchemaType::Array) => {
                if let Some(items) = &*schema.items {
                    self.generate_array_item(base_output_path, name, items.clone(), file_path)?
                }
            }
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
                        spec,
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
    ) -> Result<(String, String, bool), String> {
        //todo: check $ref
        match property {
            RefOrObject::Object(schema) => match schema.schema_type {
                None => Ok(("()".into(), "Option<()>".into(), false)),
                Some(SchemaType::Boolean) => Ok(("bool".into(), "Option<bool>".into(), false)),
                Some(SchemaType::Integer) => match schema.format {
                    None => Ok(("i64".into(), "Option<i64>".into(), false)),
                    Some(SchemaFormat::Standard(StandardFormat::Int32)) => {
                        Ok(("i32".into(), "Option<i32>".into(), false))
                    }
                    Some(SchemaFormat::Standard(StandardFormat::Int64)) => {
                        Ok(("i64".into(), "Option<i64>".into(), false))
                    }
                    Some(SchemaFormat::Standard(StandardFormat::UInt32)) => {
                        Ok(("u32".into(), "Option<u32>".into(), false))
                    }
                    Some(SchemaFormat::Standard(StandardFormat::UInt64)) => {
                        Ok(("u64".into(), "Option<u64>".into(), false))
                    }
                    Some(_) => Ok(("i64".into(), "Option<i64>".into(), false)),
                },
                Some(SchemaType::Number) => match schema.format {
                    None => Ok(("f64".into(), "Option<f64>".into(), false)),
                    Some(SchemaFormat::Standard(StandardFormat::Float)) => {
                        Ok(("f32".into(), "Option<f32>".into(), false))
                    }
                    Some(SchemaFormat::Standard(StandardFormat::Double)) => {
                        Ok(("f64".into(), "Option<f64>".into(), false))
                    }
                    Some(_) => Ok(("f64".into(), "Option<f64>".into(), false)),
                },
                Some(SchemaType::String) => match schema.format {
                    None => Ok(("String".into(), "Option<String>".into(), false)),
                    Some(SchemaFormat::Standard(StandardFormat::Date)) => {
                        dependencies.push("chrono::NaiveDate".into());
                        Ok(("NaiveDate".into(), "Option<NaiveDate>".into(), false))
                    }
                    Some(SchemaFormat::Standard(StandardFormat::DateTime)) => {
                        dependencies.push("chrono::{DateTime, Utc}".into());
                        Ok((
                            "DateTime<Utc>".into(),
                            "Option<DateTime<Utc>>".into(),
                            false,
                        ))
                    }
                    Some(SchemaFormat::Standard(StandardFormat::Uuid)) => {
                        Ok(("uuid::Uuid".into(), "Option<uuid::Uuid>".into(), false))
                    }
                    Some(_) => Ok(("String".into(), "Option<String>".into(), false)),
                },
                Some(_) => Ok(("String".into(), "Option<String>".into(), false)),
            },
            RefOrObject::Ref(r) => {
                let type_name = r
                    .reference
                    .clone()
                    .split('/')
                    .next_back()
                    .ok_or(format!(
                        "Wrong schema href {}. Expected: `#/components/schemas/{{name}}`",
                        r.reference
                    ))?
                    .to_string();

                dependencies.push(format!("crate::dto::schema::{}", type_name));
                dependencies.push(format!("crate::dto::schema::{}Raw", type_name));

                Ok((type_name.clone(), format!("Option<{}Raw>", type_name), true))
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
        spec: &SpecificationRoot,
    ) -> Result<(), String> {
        let mut properties: Vec<Prop> = vec![];
        let mut dependencies: Vec<String> = vec![];

        for (name, property) in schema_properties {
            let schema = self.schema_extractor.extract(&property, spec)?;
            let weight = match schema.extension_ui {
                Some(ref ext) => ext.weight,
                None => 0,
            };

            let (mut data_type, raw_type, is_ref) =
                self.schema_to_rs(property, &mut dependencies)?;

            let req = &required_fields.clone().unwrap_or_default();

            let required: bool;

            let clean_type = data_type.clone();

            if req.contains(&name) {
                required = true;
            } else {
                data_type = format!("Option<{}>", data_type);
                required = false;
            }

            properties.push(Prop {
                name: name.clone().to_case(Case::Snake),
                rename: name,
                clean_type,
                data_type,
                raw_type,
                required,
                is_ref,
                weight,
            })
        }

        properties.sort_by_key(|item| item.weight);

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
    ) -> Result<(), String> {
        let mut dependencies: Vec<String> = vec![];

        let (data_type, raw_type, is_ref) = self.schema_to_rs(items_type, &mut dependencies)?;

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

    fn generate_dictionary_item(
        &self,
        base_output_path: &str,
        name: &String,
        additional_type: RefOrObject<Schema>,
        file_path: String,
    ) -> Result<(), String> {
        let mut dependencies: Vec<String> = vec![];

        let (data_type, raw_type, is_ref) =
            self.schema_to_rs(additional_type, &mut dependencies)?;

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
            DICTIONARY_ITEM_TEMPLATE,
            &data,
            file_path.as_str(),
        )?;

        Ok(())
    }

    pub fn generate(&self, base_output_path: &str, spec: &SpecificationRoot) -> Result<(), String> {
        self.renderer
            .render(base_output_path, MOD_TEMPLATE, spec, MOD_PATH)?;

        for (name, schema) in &spec.components.schemas {
            self.generate_item(base_output_path, name, schema, spec)?;
        }

        Ok(())
    }
}
