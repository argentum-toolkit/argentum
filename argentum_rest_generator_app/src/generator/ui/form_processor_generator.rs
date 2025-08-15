use crate::extractor::{RequestBodyExtractor, SchemaExtractor};
use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{
    InPlace, Operation, RefOrObject, SpecificationRoot,
};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

const MOD_PATH: &str = "/src/ui/form_processor/mod.rs";
const MOD_TEMPLATE: &str = "ui/form_processor.mod";
const ITEM_TEMPLATE: &str = "ui/form_processor.item";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data<'a> {
    operation: &'a Operation,
    schema_name: Option<String>,
    need_path_params: bool,
    response_names: BTreeMap<String, String>,
}

pub(crate) struct FormProcessorGenerator {
    renderer: Arc<Renderer>,
    request_body_extractor: Arc<RequestBodyExtractor>,
    schema_extractor: Arc<SchemaExtractor>,
}

impl FormProcessorGenerator {
    pub fn new(
        renderer: Arc<Renderer>,
        request_body_extractor: Arc<RequestBodyExtractor>,
        schema_extractor: Arc<SchemaExtractor>,
    ) -> Self {
        Self {
            renderer,
            request_body_extractor,
            schema_extractor,
        }
    }

    fn generate_item(
        &self,
        base_output_path: &str,
        operation: &Operation,
        spec: &SpecificationRoot,
    ) -> Result<(), String> {
        let file_path = format!(
            "/src/ui/form_processor/{}_form_processor.rs",
            operation.operation_id.to_case(Case::Snake)
        );

        let mut response_names: BTreeMap<String, String> = BTreeMap::new();

        let need_path_params = match &operation.parameters {
            Some(params) => params.iter().any(|x| x.in_place == InPlace::Path),
            None => false,
        };

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

            response_names.insert(code.to_string(), self.escape_response_name(&response_name));
        }

        let mut schema_name: Option<String> = None;

        if let Some(request_body) = self.request_body_extractor.extract(operation, spec)? {
            //TODO copypasted from request_generator.rs
            let body = request_body
                .content
                .get("application/json")
                .ok_or("Request body should contain `application/json` mime type")?;

            if let Some((s_name, _)) = self
                .schema_extractor
                .extract_ref_with_name(&body.schema, spec)?
            {
                schema_name = Some(s_name);
            }
        };

        let data = Data {
            operation,
            schema_name,
            need_path_params,
            response_names,
        };

        self.renderer
            .render(base_output_path, ITEM_TEMPLATE, &data, file_path.as_str())?;

        Ok(())
    }

    fn escape_response_name(&self, name: &str) -> String {
        if !name.is_empty() && name[0..1].parse::<u8>().is_ok() {
            "Status".to_owned() + name
        } else {
            name.into()
        }
    }

    fn generate_mod(
        &self,
        base_output_path: &str,
        operations: Vec<Operation>,
    ) -> Result<(), String> {
        let filtered: Vec<Operation> = operations
            .into_iter()
            .filter(|o| o.extension_form.is_some())
            .collect();

        let data = HashMap::from([("operations", filtered)]);

        self.renderer
            .render(base_output_path, MOD_TEMPLATE, data, MOD_PATH)
    }

    pub fn generate(&self, base_output_path: &str, spec: &SpecificationRoot) -> Result<(), String> {
        let operations = spec.operations();
        self.generate_mod(base_output_path, operations.clone())?;

        for operation in operations.into_iter() {
            if operation.extension_form.is_some() {
                self.generate_item(base_output_path, &operation, spec)?;
            }
        }

        Ok(())
    }
}
