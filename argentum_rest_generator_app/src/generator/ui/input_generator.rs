use crate::dto::TypeDescription;
use crate::extractor::{RequestBodyExtractor, SchemaExtractor};
use crate::template::Renderer;
use crate::transformer::SchemaToTypeDescriptionTransformer;
use argentum_openapi_infrastructure::data_type::{
    ExtensionUi, Operation, RefOrObject, Schema, SchemaType, SpecificationRoot,
};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::sync::Arc;

const MOD_PATH: &str = "/src/ui/input/mod.rs";
const MOD_TEMPLATE: &str = "ui/input.mod";
const ITEM_TEMPLATE: &str = "ui/input.item";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data<'a> {
    // operation: &'a Operation,
    name: String,
    input: &'a Schema,
    inputs: BTreeMap<String, String>,
    dependencies: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Input {
    required: bool,
    name: String,
    ui: Ui,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ObjectInput {
    required: bool,
    type_name: String,
    name: String,
    ui: Ui,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Prop {
    input: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Ui {
    pub id: String,
    pub name: String,
    pub label: String,
}

impl From<Option<ExtensionUi>> for Ui {
    fn from(value: Option<ExtensionUi>) -> Self {
        //TODO: generate values or show warnings
        match value {
            None => Self {
                id: "unknown".to_string(),
                name: "unknown".to_string(),
                label: "unknown".to_string(),
            },
            Some(v) => Self {
                id: v.id.unwrap_or("unknown".to_string()),
                name: v.name.unwrap_or("unknown".to_string()),
                label: v.label.unwrap_or("unknown".to_string()),
            },
        }
    }
}

pub(crate) struct InputGenerator {
    renderer: Arc<Renderer>,
    schema_to_type_description_transformer: Arc<SchemaToTypeDescriptionTransformer>,
    request_body_extractor: Arc<RequestBodyExtractor>,
    schema_extractor: Arc<SchemaExtractor>,
}

impl InputGenerator {
    pub fn new(
        renderer: Arc<Renderer>,
        schema_to_type_description_transformer: Arc<SchemaToTypeDescriptionTransformer>,
        request_body_extractor: Arc<RequestBodyExtractor>,
        schema_extractor: Arc<SchemaExtractor>,
    ) -> Self {
        Self {
            renderer,
            schema_to_type_description_transformer,
            request_body_extractor,
            schema_extractor,
        }
    }

    fn generate_item(
        &self,
        base_output_path: &str,
        name: String,
        input: &Schema,
        spec: &SpecificationRoot,
    ) -> Result<(), Box<dyn Error>> {
        let file_path = format!("/src/ui/input/{}_input.rs", name.to_case(Case::Snake));

        let mut inputs = BTreeMap::new();
        let mut dependencies: Vec<String> = vec![];

        let req = &input.required.clone().unwrap_or_default();

        for (name, property) in input.properties.clone().unwrap_or_default() {
            let schema = self.schema_extractor.extract(&property, &spec);

            let required: bool;
            if req.contains(&name) {
                required = true;
            } else {
                required = false;
            }

            let input = match property {
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
                    dependencies.push(format!("crate::ui::input::{}Input", type_name));

                    self.renderer.render_to_result(
                        "ui/input/object",
                        ObjectInput {
                            required,
                            type_name,
                            name: name.clone(),
                            ui: schema.clone().extension_ui.into(),
                        },
                    )
                }
                RefOrObject::Object(s) => match s.schema_type {
                    Some(SchemaType::Boolean) => {
                        dependencies
                            .push("argentum_standard_ui::rsx::form::LabeledCheckbox".to_string());

                        self.renderer.render_to_result(
                            "ui/input/labeled_checkbox",
                            Input {
                                required,
                                name: name.clone(),
                                ui: schema.clone().extension_ui.into(),
                            },
                        )
                    }
                    _ => {
                        dependencies
                            .push("argentum_standard_ui::rsx::form::LabeledInput".to_string());

                        self.renderer.render_to_result(
                            "ui/input/labeled_input",
                            Input {
                                required,
                                name: name.clone(),
                                ui: schema.clone().extension_ui.into(),
                            },
                        )
                    }
                },
            }
            .unwrap_or(format!("can't render property {}", name));

            inputs.insert(name, input);
        }

        dependencies.sort();
        dependencies.dedup();

        let data = Data {
            name,
            input,
            inputs,
            dependencies,
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

    fn get_inputs(
        &self,
        ref_or: &RefOrObject<Schema>,
        spec: &SpecificationRoot,
        inputs: &mut BTreeMap<String, Schema>,
    ) {
        if let Some((schema_name, schema)) =
            self.schema_extractor.extract_ref_with_name(&ref_or, spec)
        {
            inputs.insert(schema_name, schema.clone());

            for (_, property) in schema.properties.unwrap_or_default() {
                self.get_inputs(&property, spec, inputs);
            }
        }
    }

    fn generate_mod(
        &self,
        base_output_path: &str,
        spec: &SpecificationRoot,
    ) -> Result<(), Box<dyn Error>> {
        let mut inputs: BTreeMap<String, Schema> = BTreeMap::new();

        for operation in spec.operations().into_iter() {
            if let Some(request_body) = self.request_body_extractor.extract(&operation, &spec) {
                let body = request_body
                    .content
                    .get("application/json")
                    .expect("Request body should contain `application/json` mime type");

                self.get_inputs(&body.schema, spec, &mut inputs);
            }
        }

        let data = HashMap::from([("inputs", inputs)]);

        self.renderer
            .render(base_output_path, MOD_TEMPLATE, data, MOD_PATH)
    }

    pub fn generate(
        &self,
        base_output_path: &str,
        spec: &SpecificationRoot,
    ) -> Result<(), Box<dyn Error>> {
        let mut inputs: BTreeMap<String, Schema> = BTreeMap::new();

        for operation in spec.operations().into_iter() {
            if let Some(request_body) = self.request_body_extractor.extract(&operation, &spec) {
                let body = request_body
                    .content
                    .get("application/json")
                    .expect("Request body should contain `application/json` mime type");

                self.get_inputs(&body.schema, spec, &mut inputs);
            }
        }

        self.generate_mod(base_output_path, &spec)?;

        for (name, input) in inputs.into_iter() {
            self.generate_item(base_output_path, name, &input, spec)?;
        }

        Ok(())
    }
}
