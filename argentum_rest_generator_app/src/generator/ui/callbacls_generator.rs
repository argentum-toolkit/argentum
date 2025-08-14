use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{Operation, RefOrObject, SpecificationRoot};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::sync::Arc;

const MOD_PATH: &str = "/src/ui/callbacks/mod.rs";
const MOD_TEMPLATE: &str = "ui/callbacks.mod";
const ITEM_TEMPLATE: &str = "ui/callbacks.item";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data<'a> {
    operation: &'a Operation,
    response_names: BTreeMap<String, String>,
}

pub(crate) struct CallbacksGenerator {
    renderer: Arc<Renderer>,
}

impl CallbacksGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    fn generate_item(
        &self,
        base_output_path: &str,
        operation: &Operation,
    ) -> Result<(), Box<dyn Error>> {
        let file_path = format!(
            "/src/ui/callbacks/{}_callbacks.rs",
            operation.operation_id.to_case(Case::Snake)
        );

        let mut response_names: BTreeMap<String, String> = BTreeMap::new();

        for (code, resp_or_ref) in &operation.responses {
            let response_name = match resp_or_ref {
                RefOrObject::Ref(r) => r
                    .reference
                    .clone()
                    .split('/')
                    .next_back()
                    .ok_or(format!(
                        "Wrong schema href {}. Expected: `#/components/responses/{{name}}`",
                        r.reference
                    ))?
                    .to_string(),
                RefOrObject::Object(_) => {
                    todo!(
                        "Only reference is supported currently. Inline objects in response enum are not supported yet."
                    )
                }
            };

            response_names.insert(code.to_string(), self.escape_response_name(response_name));
        }

        let data = Data {
            operation,
            response_names,
        };

        self.renderer
            .render(base_output_path, ITEM_TEMPLATE, &data, file_path.as_str())?;

        Ok(())
    }

    fn escape_response_name(&self, name: String) -> String {
        if name[0..1].parse::<u8>().is_ok() {
            "Status".to_owned() + &name
        } else {
            name
        }
    }

    fn generate_mod(
        &self,
        base_output_path: &str,
        operations: Vec<Operation>,
    ) -> Result<(), Box<dyn Error>> {
        let filtered: Vec<Operation> = operations
            .into_iter()
            .filter(|o| o.extension_form.is_some())
            .collect();

        let data = HashMap::from([("operations", filtered)]);

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
            if operation.extension_form.is_some() {
                self.generate_item(base_output_path, &operation)?;
            }
        }

        Ok(())
    }
}
