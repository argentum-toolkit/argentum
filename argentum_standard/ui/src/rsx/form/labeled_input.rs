use std::str::FromStr;

use crate::rsx::component::ErrorBlock;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
#[props(borrowed)]
pub struct InputTextProps<V: Clone + PartialEq + FromStr + 'static> {
    pub value: String,
    pub id: String,
    pub name: String,
    pub input_type: String,
    pub required: bool,
    pub label: String,
    pub violations: Option<ViolationsDto>,

    processor: LabeledInputProcessor,

    oninput: EventHandler<V>,
}

#[derive(Clone, PartialEq, Default)]
pub struct LabeledInputProcessor {}

#[component]
pub fn LabeledInput<V: Clone + PartialEq + FromStr + 'static>(props: InputTextProps<V>) -> Element {
    let label_class = match props.violations {
        Some(_) => "text-error",
        None => "",
    };

    let required_class = match props.required {
        true => "required-label",
        false => "",
    };

    let input_class = match props.violations {
        Some(_) => "input-error",
        None => "",
    };

    rsx! {
        fieldset {
            class: "fieldset w-full",
            label {
                "for":"{props.id}",
                class: "label-text {label_class} {required_class}",
                "{props.label}"
            }

            input {
                id:"{props.id}", name:"{props.name}", "type":"{props.input_type}", required: props.required,
                value: "{props.value}",
                class: "input  w-full {input_class}",
                oninput: move |event| {
                    match event.value().parse::<V>() {
                        Ok(value) => props.oninput.call(value),
                        Err(_) => {
                            // todo: Handle the error, e.g., log it or ignore it
                            // log::error!("Failed to parse input value");
                        }
                    }
                }
            }

            if let Some(v) = props.violations {
                ErrorBlock {errors: v.errors}
            }
        }
    }
}
