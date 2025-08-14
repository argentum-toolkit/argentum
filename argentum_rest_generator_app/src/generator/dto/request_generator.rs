use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{
    Operation, RefOrObject, RequestBody, SpecificationRoot,
};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::collections::HashMap;
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

    fn generate_item_with_body(
        &self,
        base_output_path: &str,
        operation: &Operation,
        request_body: RequestBody,
    ) -> Result<(), String> {
        let file_path = format!(
            "/src/dto/request/{}_request.rs",
            operation.operation_id.to_case(Case::Snake)
        );

        //TODO: support other mime types
        let body = request_body
            .content
            .get("application/json")
            .ok_or("Request body should contain `application/json` mime type")?;

        let schema = match &body.schema {
            RefOrObject::Ref(r) => r
                .reference
                .clone()
                .split('/')
                .next_back()
                .ok_or(format!(
                    "Wrong schema href {}. Expected: `#/components/schemas/{{name}}`",
                    r.reference
                ))?
                .to_string(),
            RefOrObject::Object(_o) => {
                todo!(
                    "Only reference is supported currently. Embedded objects in request are not supported yet."
                )
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
    fn generate_item_with_empty_body(
        &self,
        base_output_path: &str,
        operation: &Operation,
    ) -> Result<(), String> {
        let file_path = format!(
            "/src/dto/request/{}_request.rs",
            operation.operation_id.to_case(Case::Snake)
        );

        let data = Data {
            operation,
            body_schema: "".to_string(),
        };

        self.renderer
            .render(base_output_path, ITEM_TEMPLATE, &data, file_path.as_str())?;

        Ok(())
    }

    fn generate_mod(
        &self,
        base_output_path: &str,
        operations: Vec<Operation>,
    ) -> Result<(), String> {
        let data = HashMap::from([("operations", operations)]);

        self.renderer
            .render(base_output_path, MOD_TEMPLATE, data, MOD_PATH)
    }

    pub fn generate(&self, base_output_path: &str, spec: &SpecificationRoot) -> Result<(), String> {
        let operations = spec.operations();

        self.generate_mod(base_output_path, operations.clone())?;

        for operation in operations.into_iter() {
            //TODO: RequestBodyExtractor::extract
            match operation.clone().request_body {
                Some(b) => {
                    let request_body = match b {
                        RefOrObject::Ref(r) => {
                            let parts = r.reference.split("#/").collect::<Vec<_>>();

                            if parts.clone().len() != 2 {
                                return Err(
                                    format!("Wrong format of reference {}", r.reference).into()
                                );
                            }

                            let _file_path = parts
                                .first()
                                .ok_or(format!("Wrong file path of reference {}", r.reference))?;

                            let component_path = parts.last().ok_or(format!(
                                "Wrong component path of reference {}",
                                r.reference
                            ))?;

                            let component_parts = component_path.split('/').collect::<Vec<_>>();

                            if component_parts.clone().len() != 3
                                || component_parts[0] != "components"
                                || component_parts[1] != "requestBodies"
                            {
                                return Err(format!("Wrong component path {component_path}. Expected: `#/components/requestBodies/{{name}}`").into());
                            }

                            let component_name = component_parts.last()
                                .ok_or(format!(
                                    "Wrong component path {component_path}. Expected: `#/components/requestBodies/{{name}}`"
                                ))?;

                            spec.components.request_bodies[&component_name.to_string()].clone()
                        }
                        RefOrObject::Object(request_body) => request_body,
                    };

                    self.generate_item_with_body(base_output_path, &operation, request_body)?;
                }
                None => {
                    self.generate_item_with_empty_body(base_output_path, &operation)?;
                }
            }
        }

        Ok(())
    }
}
