use crate::extractor::{RequestBodyExtractor, SchemaExtractor};
use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{
    ExtensionUi, RefOrObject, Schema, SchemaFormat, SchemaType, SpecificationRoot, StandardFormat,
};
use convert_case::{Case, Casing};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

const MOD_PATH: &str = "/src/ui/input/mod.rs";
const MOD_TEMPLATE: &str = "ui/input.mod";
const ITEM_TEMPLATE: &str = "ui/input.item";
const OPTIONAL_ITEM_TEMPLATE: &str = "ui/optional_input.item";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data<'a> {
    name: String,
    input: &'a Schema,
    ui: Ui,
    inputs: Vec<KV>,
    dependencies: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OptionalData<'a> {
    name: String,
    input: &'a Schema,
    dependencies: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct KV {
    key: String,
    value: String,
    weight: i16,
    input_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Input {
    required: bool,
    name: String,
    title: String,
    ui: Ui,
    value_type: String,
    input_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ObjectInput {
    required: bool,
    type_name: String,
    name: String,
    title: Option<String>,
    ui: Ui,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct Ui {
    pub id: String,
    pub name: String,
    pub label: String,
    pub wrapper: String,
    pub optional_title: Option<String>,
    pub optional_style: String,
}

impl From<Option<ExtensionUi>> for Ui {
    fn from(value: Option<ExtensionUi>) -> Self {
        //TODO: generate values or show warnings
        match value {
            None => Self {
                id: "unknown".into(),
                name: "unknown".into(),
                label: "unknown".into(),
                wrapper: "none".into(),
                optional_title: None,
                optional_style: "none".into(),
            },
            Some(v) => Self {
                id: v.id.unwrap_or("unknown".into()),
                name: v.name.unwrap_or("unknown".into()),
                label: v.label.unwrap_or("unknown".into()),
                wrapper: v.wrapper.into(),
                optional_title: v.optional_title,
                optional_style: v.optional_style.into(),
            },
        }
    }
}

impl From<ExtensionUi> for Ui {
    fn from(value: ExtensionUi) -> Self {
        //TODO: generate values or show warnings
        Self {
            id: value.id.unwrap_or("unknown".into()),
            name: value.name.unwrap_or("unknown".into()),
            label: value.label.unwrap_or("unknown".into()),
            wrapper: value.wrapper.into(),
            optional_title: value.optional_title,
            optional_style: value.optional_style.into(),
        }
    }
}

pub(crate) struct InputGenerator {
    renderer: Arc<Renderer>,
    request_body_extractor: Arc<RequestBodyExtractor>,
    schema_extractor: Arc<SchemaExtractor>,
}

impl InputGenerator {
    fn input_type(schema: &Schema) -> String {
        match schema.format.as_ref() {
            Some(SchemaFormat::Standard(StandardFormat::Password)) => "password".into(),
            _ => "text".into(),
        }
    }

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
        name: String,
        input: &Schema,
        spec: &SpecificationRoot,
    ) -> Result<(), String> {
        let file_path = format!("/src/ui/input/{}_input.rs", name.to_case(Case::Snake));
        let optional_file_path = format!(
            "/src/ui/input/{}_optional_input.rs",
            name.to_case(Case::Snake)
        );

        let mut inputs = Vec::new();
        let mut dependencies: Vec<String> = vec![];

        let req = &input.required.clone().unwrap_or_default();

        let mut old_weight = 0;
        for (name, property) in input.properties.clone().unwrap_or_default() {
            let schema = self.schema_extractor.extract(&property, spec)?;

            let required = req.contains(&name);

            let weight = match schema.extension_ui {
                Some(ref ext) => match ext.weight {
                    Some(w) => w,
                    None => {
                        old_weight += 1;
                        old_weight
                    }
                },
                None => {
                    old_weight += 1;
                    old_weight
                }
            };

            let input = match property {
                RefOrObject::Ref(r) => {
                    let type_name: String = r
                        .reference
                        .clone()
                        .split('/')
                        .next_back()
                        .ok_or(format!(
                            "Wrong schema href {}. Expected: `#/components/schemas/{{name}}`",
                            r.reference
                        ))?
                        .into();

                    dependencies.push(format!("crate::dto::schema::{}", type_name));
                    if required {
                        dependencies.push(format!("crate::ui::input::{}Input", type_name));
                    } else {
                        dependencies.push(format!("crate::ui::input::{}OptionalInput", type_name));
                    }
                    dependencies.push(format!("crate::ui::input::{}InputProcessor", type_name));

                    let title = match r.title {
                        Some(t) => Some(t),
                        None => schema.title.clone(),
                    };

                    let ui: Ui = match r.extension_ui {
                        Some(ext) => ext.into(),
                        None => schema.clone().extension_ui.into(),
                    };

                    let input = self
                        .renderer
                        .render_to_result(
                            "ui/input/object",
                            ObjectInput {
                                required,
                                type_name: type_name.clone(),
                                name: name.clone(),
                                ui,
                                title,
                            },
                        )
                        .unwrap_or(format!("Can't render property {}", name));

                    KV {
                        key: name,
                        value: input,
                        weight,
                        input_name: format!("{}Input", type_name),
                    }
                }
                RefOrObject::Object(s) => match s.schema_type {
                    Some(SchemaType::Boolean) => {
                        dependencies
                            .push("argentum_standard_ui::rsx::form::LabeledCheckbox".into());

                        dependencies.push(
                            "argentum_standard_ui::rsx::form::LabeledCheckboxProcessor".into(),
                        );

                        let input = self
                            .renderer
                            .render_to_result(
                                "ui/input/labeled_checkbox",
                                Input {
                                    required,
                                    name: name.clone(),
                                    title: schema.title.clone().unwrap_or_default(),
                                    input_type: Self::input_type(&schema),
                                    ui: schema.extension_ui.into(),
                                    value_type: "bool".into(),
                                },
                            )
                            .unwrap_or(format!("Can't render property {}", name));

                        KV {
                            key: name,
                            value: input,
                            weight,
                            input_name: "LabeledCheckbox".into(),
                        }
                    }
                    Some(SchemaType::Integer) => match s.format {
                        Some(SchemaFormat::Standard(StandardFormat::Int32)) => {
                            dependencies
                                .push("argentum_standard_ui::rsx::form::LabeledInput".into());

                            dependencies.push(
                                "argentum_standard_ui::rsx::form::LabeledInputProcessor".into(),
                            );

                            let input = self
                                .renderer
                                .render_to_result(
                                    "ui/input/labeled_input",
                                    Input {
                                        required,
                                        name: name.clone(),
                                        title: schema.title.clone().unwrap_or(String::new()),
                                        input_type: Self::input_type(&schema),
                                        ui: schema.extension_ui.into(),
                                        value_type: "i32".into(),
                                    },
                                )
                                .unwrap_or(format!("Can't render property {}", name));

                            KV {
                                key: name,
                                value: input,
                                weight,
                                input_name: "LabeledInput".into(),
                            }
                        }
                        _ => {
                            dependencies
                                .push("argentum_standard_ui::rsx::form::LabeledInput".into());

                            dependencies.push(
                                "argentum_standard_ui::rsx::form::LabeledInputProcessor".into(),
                            );

                            let input = self
                                .renderer
                                .render_to_result(
                                    "ui/input/labeled_input",
                                    Input {
                                        required,
                                        name: name.clone(),
                                        title: schema.title.clone().unwrap_or_default(),
                                        input_type: Self::input_type(&schema),
                                        ui: schema.extension_ui.into(),
                                        value_type: "i64".into(),
                                    },
                                )
                                .unwrap_or(format!("Can't render property {}", name));

                            KV {
                                key: name,
                                value: input,
                                weight,
                                input_name: "LabeledInput".into(),
                            }
                        }
                    },
                    Some(SchemaType::String) => match s.format {
                        Some(SchemaFormat::Standard(StandardFormat::Text)) => {
                            dependencies
                                .push("argentum_standard_ui::rsx::form::LabeledTextarea".into());

                            dependencies.push(
                                "argentum_standard_ui::rsx::form::LabeledTextareaProcessor".into(),
                            );

                            let input = self
                                .renderer
                                .render_to_result(
                                    "ui/input/labeled_textarea",
                                    Input {
                                        required,
                                        name: name.clone(),
                                        title: schema.title.clone().unwrap_or_default(),
                                        input_type: Self::input_type(&schema),
                                        ui: schema.extension_ui.into(),
                                        value_type: "String".into(),
                                    },
                                )
                                .unwrap_or(format!("Can't render property {}", name));

                            KV {
                                key: name,
                                value: input,
                                weight,
                                input_name: "LabeledTextarea".into(),
                            }
                        }
                        Some(SchemaFormat::Standard(StandardFormat::Uuid)) => {
                            dependencies
                                .push("argentum_standard_ui::rsx::form::AutocompleteUuid".into());

                            dependencies.push(
                                "argentum_standard_ui::rsx::form::AutocompleteUuidProcessor".into(),
                            );

                            let input = self
                                .renderer
                                .render_to_result(
                                    "ui/input/autocomplete_uuid",
                                    Input {
                                        required,
                                        name: name.clone(),
                                        title: schema.title.clone().unwrap_or_default(),
                                        input_type: Self::input_type(&schema),
                                        ui: schema.extension_ui.into(),
                                        value_type: "uuid::Uuid".into(),
                                    },
                                )
                                .unwrap_or(format!("Can't render property {}", name));

                            KV {
                                key: name,
                                value: input,
                                weight,
                                input_name: "AutocompleteUuid".into(),
                            }
                        }
                        _ => {
                            dependencies
                                .push("argentum_standard_ui::rsx::form::LabeledInput".into());

                            dependencies.push(
                                "argentum_standard_ui::rsx::form::LabeledInputProcessor".into(),
                            );

                            let input = self
                                .renderer
                                .render_to_result(
                                    "ui/input/labeled_input",
                                    Input {
                                        required,
                                        name: name.clone(),
                                        title: schema.title.clone().unwrap_or_default(),
                                        input_type: Self::input_type(&schema),
                                        ui: schema.extension_ui.into(),
                                        value_type: "String".into(),
                                    },
                                )
                                .unwrap_or(format!("Can't render property {}", name));

                            KV {
                                key: name,
                                value: input,
                                weight,
                                input_name: "LabeledInput".into(),
                            }
                        }
                    },
                    _ => {
                        dependencies.push("argentum_standard_ui::rsx::form::LabeledInput".into());
                        dependencies
                            .push("argentum_standard_ui::rsx::form::LabeledInputProcessor".into());

                        let input = self
                            .renderer
                            .render_to_result(
                                "ui/input/labeled_input",
                                Input {
                                    required,
                                    name: name.clone(),
                                    title: schema.title.clone().unwrap_or(String::new()),
                                    input_type: Self::input_type(&schema),
                                    ui: schema.extension_ui.into(),
                                    value_type: "String".into(),
                                },
                            )
                            .unwrap_or(format!("Can't render property {}", name));

                        KV {
                            key: name,
                            value: input,
                            weight,
                            input_name: "LabeledInput".into(),
                        }
                    }
                },
            };

            inputs.push(input);
        }

        dependencies.sort();
        dependencies.dedup();

        inputs.sort_by_key(|item| item.weight);

        let ui: Ui = input.clone().extension_ui.into();

        let data = Data {
            name: name.clone(),
            input,
            ui,
            inputs,
            dependencies: dependencies.clone(),
        };

        self.renderer
            .render(base_output_path, ITEM_TEMPLATE, &data, file_path.as_str())?;

        let optional_data = OptionalData {
            name,
            dependencies,
            input,
        };

        self.renderer.render(
            base_output_path,
            OPTIONAL_ITEM_TEMPLATE,
            &optional_data,
            optional_file_path.as_str(),
        )?;

        Ok(())
    }

    fn get_inputs(
        &self,
        ref_or: &RefOrObject<Schema>,
        spec: &SpecificationRoot,
        inputs: &mut BTreeMap<String, Schema>,
    ) -> Result<(), String> {
        if let Some((schema_name, schema)) =
            self.schema_extractor.extract_ref_with_name(ref_or, spec)?
        {
            inputs.insert(schema_name, schema.clone());

            for (_, property) in schema.properties.unwrap_or_default() {
                self.get_inputs(&property, spec, inputs)?;
            }
        }

        Ok(())
    }

    fn generate_mod(&self, base_output_path: &str, spec: &SpecificationRoot) -> Result<(), String> {
        let mut inputs: BTreeMap<String, Schema> = BTreeMap::new();

        for operation in spec.operations().into_iter() {
            if operation.extension_form.is_some()
                && let Some(request_body) = self.request_body_extractor.extract(&operation, spec)?
            {
                let body = request_body
                    .content
                    .get("application/json")
                    .ok_or("Request body should contain `application/json` mime type")?;

                self.get_inputs(&body.schema, spec, &mut inputs)?;
            }
        }

        let data = HashMap::from([("inputs", inputs)]);

        self.renderer
            .render(base_output_path, MOD_TEMPLATE, data, MOD_PATH)
    }

    pub fn generate(&self, base_output_path: &str, spec: &SpecificationRoot) -> Result<(), String> {
        let mut inputs: BTreeMap<String, Schema> = BTreeMap::new();

        for operation in spec.operations().into_iter() {
            if operation.extension_form.is_some()
                && let Some(request_body) = self.request_body_extractor.extract(&operation, spec)?
            {
                let body = request_body
                    .content
                    .get("application/json")
                    .ok_or("Request body should contain `application/json` mime type")?;

                self.get_inputs(&body.schema, spec, &mut inputs)?;
            }
        }

        self.generate_mod(base_output_path, spec)?;

        for (name, input) in inputs.into_iter() {
            self.generate_item(base_output_path, name, &input, spec)?;
        }

        Ok(())
    }
}
