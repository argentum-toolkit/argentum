use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{
    RefOrObject, Response, Schema, SpecificationRoot,
};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::sync::Arc;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    response_name: String,
    content: BTreeMap<String, String>,
}

pub(crate) struct ResponseGenerator {
    renderer: Arc<Renderer>,
}
const MOD_PATH: &str = "/src/dto/response/mod.rs";
const MOD_TEMPLATE: &str = "dto/response.mod";
const ITEM_TEMPLATE: &str = "dto/response.item";

impl ResponseGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    fn generate_item(
        &self,
        response_name: String,
        response: &Response,
        // request_body: RequestBody,
    ) -> Result<(), Box<dyn Error>> {
        let file_path = format!(
            "/src/dto/response/{}_response.rs",
            response_name.to_case(Case::Snake)
        );

        let mut content: BTreeMap<String, String> = BTreeMap::new();

        for (name, media_type) in &response.content {
            let schema_type = self.schema_to_rs(&media_type.schema);

            content.insert(name.clone(), schema_type);
        }

        let data = Data {
            response_name,
            content,
            // body_schema,
        };

        self.renderer
            .render(ITEM_TEMPLATE, &data, file_path.as_str())?;

        Ok(())
    }

    fn schema_to_rs(&self, schema: &RefOrObject<Schema>) -> String {
        let schema = match schema {
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
                todo!("Only reference is supported currently. Onboarded objects are not supported yet.")
            }
        };

        format!("crate::dto::schema::{}", schema)
    }

    fn generate_mod(&self, responses: BTreeMap<String, Response>) -> Result<(), Box<dyn Error>> {
        let mut response_names: Vec<String> = Vec::new();

        for (name, _) in responses {
            response_names.push(self.escape_response_name(name));
        }

        let data = HashMap::from([("responseNames", response_names)]);

        self.renderer.render(MOD_TEMPLATE, data, MOD_PATH)
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

    pub fn generate(&self, spec: &SpecificationRoot) -> Result<(), Box<dyn Error>> {
        let responses = spec.clone().components.responses;

        self.generate_mod(responses.clone())?;

        for (name, response) in responses.into_iter() {
            self.generate_item(self.escape_response_name(name), &response)?;
        }

        Ok(())
    }
}
