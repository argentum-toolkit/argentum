use argentum_openapi_infrastructure::data_type::{
    RefOrObject, Schema, SchemaFormat, SchemaType, StandardFormat,
};

use crate::dto::TypeDescription;

pub struct SchemaToTypeDescriptionTransformer {}

impl SchemaToTypeDescriptionTransformer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(
        &self,
        property: RefOrObject<Schema>,
        dependencies: &mut Vec<String>,
    ) -> TypeDescription {
        match property {
            RefOrObject::Object(schema) => match schema.schema_type {
                None => TypeDescription::new("()", "Option<()>", "()", false),
                Some(SchemaType::Boolean) => {
                    TypeDescription::new("bool", "Option<bool>", "bool::default()", false)
                }

                Some(SchemaType::Integer) => match schema.format {
                    None => TypeDescription::new("", "Option<i64>", "i64::default()", false),
                    Some(SchemaFormat::Standard(StandardFormat::Int32)) => {
                        TypeDescription::new("i32", "Option<i32>", "i32::default()", false)
                    }
                    Some(SchemaFormat::Standard(StandardFormat::Int64)) => {
                        TypeDescription::new("i64", "Option<i64>", "i64::default()", false)
                    }
                    Some(SchemaFormat::Standard(StandardFormat::UInt32)) => {
                        TypeDescription::new("u32", "Option<u32>", "i32::default()", false)
                    }
                    Some(SchemaFormat::Standard(StandardFormat::UInt64)) => {
                        TypeDescription::new("u64", "Option<u64>", "u64::default()", false)
                    }
                    Some(_) => TypeDescription::new("i64", "Option<i64>", "i64::default()", false),
                },
                Some(SchemaType::Number) => match schema.format {
                    None => TypeDescription::new("f64", "Option<f64>", "f64::default()", false),
                    Some(SchemaFormat::Standard(StandardFormat::Float)) => {
                        TypeDescription::new("f32", "Option<f32>", "f32::default()", false)
                    }
                    Some(SchemaFormat::Standard(StandardFormat::Double)) => {
                        TypeDescription::new("f64", "Option<f64>", "f64::default()", false)
                    }
                    Some(_) => TypeDescription::new("i64", "Option<i64>", "i64::default()", false),
                },
                Some(SchemaType::String) => match schema.format {
                    None => {
                        TypeDescription::new("String", "Option<String>", "String::default()", false)
                    }
                    Some(SchemaFormat::Standard(StandardFormat::Date)) => {
                        dependencies.push("chrono::NaiveDate".to_string());
                        TypeDescription::new(
                            "NaiveDate",
                            "Option<NaiveDate>",
                            "NaiveDate::default()",
                            false,
                        )
                    }
                    Some(SchemaFormat::Standard(StandardFormat::DateTime)) => {
                        dependencies.push("chrono::{DateTime, Utc}".to_string());
                        TypeDescription::new(
                            "DateTime<Utc>",
                            "Option<DateTime<Utc>>",
                            "Utc::now()",
                            false,
                        )
                    }
                    Some(SchemaFormat::Standard(StandardFormat::Uuid)) => TypeDescription::new(
                        "uuid::Uuid",
                        "Option<uuid::Uuid>",
                        "uuid::Uuid::default()",
                        false,
                    ),
                    Some(_) => {
                        TypeDescription::new("String", "Option<String>", "String::default()", false)
                    }
                },
                Some(_) => TypeDescription::new("()", "Option<()>", "()", false),
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

                TypeDescription::new(
                    type_name.clone(),
                    format!("Option<{}Raw>", type_name),
                    format!("{}::default()", type_name),
                    true,
                )
            }
        }
    }
}
