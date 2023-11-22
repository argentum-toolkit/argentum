use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{Operation, RefOrObject, SpecificationRoot};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::sync::Arc;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data<'a> {
    response_names: BTreeMap<String, String>,
    operation: &'a Operation,
}

pub(crate) struct OperationResponseEnumGenerator {
    renderer: Arc<Renderer>,
}

const MOD_PATH: &str = "/src/dto/operation_response_enum/mod.rs";
const MOD_TEMPLATE: &str = "dto/operation_response_enum.mod";
const ITEM_TEMPLATE: &str = "dto/operation_response_enum.item";

impl OperationResponseEnumGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    fn escape_response_name(&self, name: String) -> String {
        let response_name;

        if name[0..1].parse::<u8>().is_ok() {
            response_name = "Status".to_owned() + &name;
        } else {
            response_name = name;
        }

        response_name
    }

    fn generate_item(
        &self,
        base_output_path: &str,
        operation: &Operation,
    ) -> Result<(), Box<dyn Error>> {
        let file_path = format!(
            "/src/dto/operation_response_enum/{}_operation_response_enum.rs",
            operation.operation_id.to_case(Case::Snake)
        );

        let mut response_names: BTreeMap<String, String> = BTreeMap::new();

        for (code, resp_or_ref) in &operation.responses {
            let response_name = match resp_or_ref {
                RefOrObject::Ref(r) => r
                    .reference
                    .clone()
                    .split('/')
                    .last()
                    .unwrap_or_else(|| {
                        panic!(
                            "Wrong schema href {}. Expected: `#/components/responses/{{name}}`",
                            r.reference
                        )
                    })
                    .to_string(),
                RefOrObject::Object(_) => {
                    todo!("Only reference is supported currently. Onboarded objects are not supported yet.")
                }
            };

            response_names.insert(code.to_string(), self.escape_response_name(response_name));
        }

        let data = Data {
            response_names,
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
            self.generate_item(base_output_path, &operation)?;
        }

        Ok(())
    }
}
