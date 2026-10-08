use std::collections::BTreeMap;

use crate::dto::schema::RequestRestoreTokenSchema;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_standard_ui::rsx::form::LabeledInput;
use argentum_standard_ui::rsx::form::LabeledInputProcessor;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct RequestRestoreTokenSchemaProps {
    pub value: RequestRestoreTokenSchema,
    pub violations: Option<ViolationsDto>,
    pub ui_wrapper: String,
    pub title: Option<String>,

    processor: RequestRestoreTokenSchemaInputProcessor,

    oninput: EventHandler<RequestRestoreTokenSchema>,
}

#[derive(Clone, PartialEq, Default)]
pub struct RequestRestoreTokenSchemaInputProcessor {
    pub email_processor: LabeledInputProcessor,
}

#[component]
pub fn RequestRestoreTokenSchemaInput(props: RequestRestoreTokenSchemaProps) -> Element {
    let label_class = match props.violations {
        Some(_) => "text-error",
        None => "",
    };

    let RequestRestoreTokenSchemaProps {
        value,
        violations,
        processor,
        ui_wrapper,
        title,
        ..
    } = props.clone();

    let mut email_value = use_signal(|| value.email);

    let violation_items = match violations {
        Some(vv) => match vv.items {
            Some(ViolationItemDto::Object(items)) => items,
            _ => BTreeMap::new(),
        },
        None => BTreeMap::new(),
    };

    let email_violations = violation_items.get("email").cloned();

    let update = move || {
        props
            .oninput
            .call(RequestRestoreTokenSchema::new(email_value()))
    };

    if ui_wrapper.eq("flex") {
        rsx! {
                    fieldset {
                        if let Some(t) = title {
                            legend { class: "fieldset-legend {label_class}", "{t}"}
                        }
                        div {
                            class: "flex gap-2",
                            LabeledInput {
            id: "unknown",
            name: "unknown",
            label: "",
            input_type: "text".to_string(),
            required: true,
                value: email_value(),

            violations: email_violations,
            oninput: move |v: String| {
                email_value.set(v);
                update();
            },

            processor: processor.email_processor,
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
                            LabeledInput {
            id: "unknown",
            name: "unknown",
            label: "",
            input_type: "text".to_string(),
            required: true,
                value: email_value(),

            violations: email_violations,
            oninput: move |v: String| {
                email_value.set(v);
                update();
            },

            processor: processor.email_processor,
        }

                        }
                    }
                }
    } else {
        rsx! {
                    LabeledInput {
            id: "unknown",
            name: "unknown",
            label: "",
            input_type: "text".to_string(),
            required: true,
                value: email_value(),

            violations: email_violations,
            oninput: move |v: String| {
                email_value.set(v);
                update();
            },

            processor: processor.email_processor,
        }

                }
    }
}
