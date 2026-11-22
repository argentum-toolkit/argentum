use crate::extractor::{RequestBodyExtractor, SchemaExtractor};
use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{
    InPlace, Operation, Path, RefOrObject, SpecificationRoot,
};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::collections::HashMap;
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
    ui_wrapper: String,
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
        path: &Path,
        spec: &SpecificationRoot,
    ) -> Result<(), String> {
        let file_path = format!(
            "/src/ui/form/{}_form.rs",
            operation.operation_id.to_case(Case::Snake)
        );

        let need_path_params = match &operation.parameters {
            Some(params) => params.iter().any(|x| x.in_place == InPlace::Path),
            None => match &path.parameters {
                Some(params) => params.iter().any(|x| x.in_place == InPlace::Path),
                None => false,
            },
        };

        let mut schema_name: Option<String> = None;
        let mut ui_wrapper = "none".to_string();

        if let Some(request_body) = self.request_body_extractor.extract(operation, spec)? {
            // TODO: copypasted from request_generator.rs
            let body = request_body
                .content
                .get("application/json")
                .ok_or("Request body should contain `application/json` mime type")?;

            if let Some(extension_ui) = &body.extension_ui {
                ui_wrapper = extension_ui.wrapper.clone().into();
            }

            if let Some((s_name, schema)) = self
                .schema_extractor
                .extract_ref_with_name(&body.schema, spec)?
            {
                schema_name = Some(s_name);

                if body.extension_ui.is_none() {
                    if let Some(extension_ui) = schema.extension_ui {
                        ui_wrapper = extension_ui.wrapper.into();
                    }
                }
            } else if let RefOrObject::Object(schema) = &body.schema {
                if let Some(extension_ui) = schema.extension_ui.clone() {
                    ui_wrapper = extension_ui.wrapper.into();
                }
            }
        };

        let data = Data {
            need_path_params,
            operation,
            schema_name,
            ui_wrapper,
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
        let filtered: Vec<Operation> = operations
            .into_iter()
            .filter(|o| o.extension_form.is_some())
            .collect();

        let data = HashMap::from([("operations", filtered)]);

        self.renderer
            .render(base_output_path, MOD_TEMPLATE, data, MOD_PATH)
    }

    pub fn generate(&self, base_output_path: &str, spec: &SpecificationRoot) -> Result<(), String> {
        for (_, path) in spec.paths.clone().into_iter() {
            for (_, operation) in path.clone().operations.into_iter() {
                if operation.extension_form.is_some() {
                    self.generate_item(base_output_path, &operation, &path, spec)?;
                }
            }
        }

        self.generate_mod(base_output_path, spec.operations().clone())?;

        Ok(())
    }
}
