use crate::extractor::{RequestBodyExtractor, SchemaExtractor};
use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{InPlace, Operation, SpecificationRoot};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

const MOD_PATH: &str = "/src/ui/form/mod.rs";
const MOD_TEMPLATE: &str = "ui/form.mod";
const ITEM_TEMPLATE: &str = "ui/form.item";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data<'a> {
    need_path_params: bool,
    operation: &'a Operation,
    schema_name: Option<String>,
}

pub(crate) struct FormGenerator {
    renderer: Arc<Renderer>,
    request_body_extractor: Arc<RequestBodyExtractor>,
    schema_extractor: Arc<SchemaExtractor>,
}

impl FormGenerator {
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
    ) -> Result<(), Box<dyn Error>> {
        let file_path = format!(
            "/src/ui/form/{}_form.rs",
            operation.operation_id.to_case(Case::Snake)
        );

        let need_path_params = match &operation.parameters {
            Some(params) => match params.iter().find(|&x| x.in_place == InPlace::Path) {
                Some(_) => true,
                None => false,
            },
            None => false,
        };

        let mut schema_name: Option<String> = None;

        if let Some(request_body) = self.request_body_extractor.extract(operation, &spec) {
            //TODO copypasted from request_generator.rs
            let body = request_body
                .content
                .get("application/json")
                .expect("Request body should contain `application/json` mime type");

            if let Some((s_name, _)) = self
                .schema_extractor
                .extract_ref_with_name(&body.schema, spec)
            {
                schema_name = Some(s_name);
            }
        };

        let data = Data {
            need_path_params,
            operation,
            schema_name,
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
            self.generate_item(base_output_path, &operation, spec)?;
        }

        Ok(())
    }
}
