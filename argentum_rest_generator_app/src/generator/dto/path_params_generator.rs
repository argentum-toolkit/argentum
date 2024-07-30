use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{
    Operation, Parameter, RefOrObject, SchemaFormat, SchemaType, SpecificationRoot, StandardFormat,
};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data<'a> {
    operation: &'a Operation,
    dependencies: Vec<String>,
    properties: Vec<Prop>,
}

pub(crate) struct PathParamsGenerator {
    renderer: Arc<Renderer>,
}

//TODO: fix copy/paste from schema
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

const MOD_PATH: &str = "/src/dto/path_params/mod.rs";
const MOD_TEMPLATE: &str = "dto/path_params.mod";
const ITEM_TEMPLATE: &str = "dto/path_params.item";

impl PathParamsGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    fn generate_item(
        &self,
        base_output_path: &str,
        operation: &Operation,
        uri_parameters: &Option<Vec<Parameter>>,
    ) -> Result<(), Box<dyn Error>> {
        let file_path = format!(
            "/src/dto/path_params/{}_path_params.rs",
            operation.operation_id.to_case(Case::Snake)
        );

        let mut properties: Vec<Prop> = vec![];
        let mut dependencies: Vec<String> = vec![];

        let mut parameters: Vec<Parameter> = vec![];

        //TODO: add path_parameters from path.rs
        match uri_parameters {
            Some(params) => {
                for param in params {
                    parameters.push(param.clone())
                }
            }
            None => {}
        };

        match &operation.parameters {
            Some(params) => {
                for param in params {
                    parameters.push(param.clone())
                }
            }
            None => {}
        }

        for parameter in parameters {
            let property = parameter.schema.clone();
            let name = parameter.name;
            //todo: check $ref
            let (data_type, raw_type, is_ref) = match property {
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

            properties.push(Prop {
                name: name.clone().to_case(Case::Snake),
                rename: name,
                data_type,
                raw_type,
                required: parameter.required,
                is_ref,
            })
        }

        let data = Data {
            dependencies,
            properties,
            operation,
        };

        self.renderer
            .render(base_output_path, ITEM_TEMPLATE, &data, file_path.as_str())?;

        Ok(())
    }

    fn generate_mod(
        &self,
        base_output_path: &str,
        operations: Vec<Operation>,
    ) -> Result<(), Box<dyn Error>> {
        let data = HashMap::from([("operations", operations.as_slice())]);

        self.renderer
            .render(base_output_path, MOD_TEMPLATE, data, MOD_PATH)
    }

    pub fn generate(
        &self,
        base_output_path: &str,
        spec: &SpecificationRoot,
    ) -> Result<(), Box<dyn Error>> {
        let operations = spec.operations();
        self.generate_mod(base_output_path, operations.clone())?;

        for path in spec.paths.values() {
            let operations = &path.operations;

            for (_method, operation) in operations.iter() {
                self.generate_item(base_output_path, operation, &path.parameters)?;
            }
        }

        Ok(())
    }
}
