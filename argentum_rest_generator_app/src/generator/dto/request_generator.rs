use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{
    Operation, RefOrObject, RequestBody, SpecificationRoot,
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
    body_schema: String,
}

pub(crate) struct RequestGenerator {
    renderer: Arc<Renderer>,
}

const MOD_PATH: &str = "/src/dto/request/mod.rs";
const MOD_TEMPLATE: &str = "dto/request.mod";
const ITEM_TEMPLATE: &str = "dto/request.item";

impl RequestGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    fn generate_item(
        &self,
        base_output_path: &str,
        operation: &Operation,
        request_body: RequestBody,
    ) -> Result<(), Box<dyn Error>> {
        let file_path = format!(
            "/src/dto/request/{}_request.rs",
            operation.operation_id.to_case(Case::Snake)
        );

        //TODO: support other mime types
        let body = request_body
            .content
            .get("application/json")
            .expect("Request body should contain `application/json` mime type");

        let schema = match &body.schema {
            RefOrObject::Ref(r) => r
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
                .to_string(),
            RefOrObject::Object(_o) => {
                todo!("generate onboarded objects")
            }
        };

        let body_schema = format!("crate::dto::schema::{}", schema);

        let data = Data {
            operation,
            body_schema,
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
        let data = HashMap::from([("operations", operations)]);

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

        for operation in operations.into_iter() {
            if operation.request_body.is_some() {
                let request_body = match operation.clone().request_body.unwrap() {
                    RefOrObject::Ref(r) => {
                        let parts = r.reference.split("#/").collect::<Vec<_>>();

                        if parts.clone().len() != 2 {
                            panic!("Wrong format of reference {}", r.reference)
                        }

                        let _file_path = parts.first().unwrap_or_else(|| {
                            panic!("Wrong file path of reference {}", r.reference)
                        });

                        let component_path = parts.last().unwrap_or_else(|| {
                            panic!("Wrong component path of reference {}", r.reference)
                        });

                        let component_parts = component_path.split('/').collect::<Vec<_>>();

                        if component_parts.clone().len() != 3
                            || component_parts[0] != "components"
                            || component_parts[1] != "requestBodies"
                        {
                            panic!(
                                "Wrong component path {}. Expected: `#/components/requestBodies/{{name}}`",
                                component_path
                            )
                        }

                        let component_name = component_parts.last().unwrap_or_else(|| panic!(
                            "Wrong component path {}. Expected: `#/components/requestBodies/{{name}}`",
                            component_path
                        ));

                        spec.components.request_bodies[&component_name.to_string()].clone()
                    }
                    RefOrObject::Object(request_body) => request_body,
                };

                self.generate_item(base_output_path, &operation, request_body)?;
            }
        }

        Ok(())
    }
}
