use std::collections::BTreeMap;

use crate::dto::schema::ChangePasswordSchema;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_standard_ui::rsx::form::LabeledInput;
use argentum_standard_ui::rsx::form::LabeledInputProcessor;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ChangePasswordSchemaProps {
    pub value: ChangePasswordSchema,
    pub violations: Option<ViolationsDto>,
    pub ui_wrapper: String,
    pub title: Option<String>,

    processor: ChangePasswordSchemaInputProcessor,

    oninput: EventHandler<ChangePasswordSchema>,
}

#[derive(Clone, PartialEq, Default)]
pub struct ChangePasswordSchemaInputProcessor {
    pub token_processor: LabeledInputProcessor,
    pub password_processor: LabeledInputProcessor,
}

#[component]
pub fn ChangePasswordSchemaInput(props: ChangePasswordSchemaProps) -> Element {
    let label_class = match props.violations {
        Some(_) => "text-error",
        None => "",
    };

    let ChangePasswordSchemaProps {
        value,
        violations,
        processor,
        ui_wrapper,
        title,
        ..
    } = props.clone();

    let mut token_value = use_signal(|| value.token);
    let mut password_value = use_signal(|| value.password);

    let violation_items = match violations {
        Some(vv) => match vv.items {
            Some(ViolationItemDto::Object(items)) => items,
            _ => BTreeMap::new(),
        },
        None => BTreeMap::new(),
    };

    let password_violations = violation_items.get("password").cloned();

    let update = move || {
        props
            .oninput
            .call(ChangePasswordSchema::new(token_value(), password_value()))
    };

    if ui_wrapper.eq("flex") {
        rsx! {
                    fieldset {
                        if let Some(t) = title {
                            legend { class: "fieldset-legend {label_class}", "{t}"}
                        }
                        div {
                            class: "flex gap-2",
                            input {
            id: "token",
            name: "token",
            r#type: "hidden",
            required: true,
                value: token_value(),
            oninput: move |event| {
                let value = event.value().parse::<String>().unwrap_or_default();
                token_value.set(value);
                update();
            },
        }

                            LabeledInput {
            id: "unknown",
            name: "unknown",
            label: "",
            input_type: "password".to_string(),
            required: true,
                value: password_value(),

            violations: password_violations,
            oninput: move |v: String| {
                password_value.set(v);
                update();
            },

            processor: processor.password_processor,
        }

                        }
                    }
                }
    } else if ui_wrapper.eq("block") {
        rsx! {
                    fieldset {
                        if let Some(t) = title {
                            legend { class: "fieldset-legend {label_class}", "{t}"}
                        }
                        div {
                            input {
            id: "token",
            name: "token",
            r#type: "hidden",
            required: true,
                value: token_value(),
            oninput: move |event| {
                let value = event.value().parse::<String>().unwrap_or_default();
                token_value.set(value);
                update();
            },
        }

                            LabeledInput {
            id: "unknown",
            name: "unknown",
            label: "",
            input_type: "password".to_string(),
            required: true,
                value: password_value(),

            violations: password_violations,
            oninput: move |v: String| {
                password_value.set(v);
                update();
            },

            processor: processor.password_processor,
        }

                        }
                    }
                }
    } else {
        rsx! {
                    input {
            id: "token",
            name: "token",
            r#type: "hidden",
            required: true,
                value: token_value(),
            oninput: move |event| {
                let value = event.value().parse::<String>().unwrap_or_default();
                token_value.set(value);
                update();
            },
        }

                    LabeledInput {
            id: "unknown",
            name: "unknown",
            label: "",
            input_type: "password".to_string(),
            required: true,
                value: password_value(),

            violations: password_violations,
            oninput: move |v: String| {
                password_value.set(v);
                update();
            },

            processor: processor.password_processor,
        }

                }
    }
}
