use crate::rsx::component::ErrorBlock;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
#[props(borrowed)]
pub struct InputTextareaProps {
    pub value: String,
    pub id: String,
    pub name: String,
    pub input_type: String,
    pub required: bool,
    pub label: String,
    pub violations: Option<ViolationsDto>,

    processor: LabeledTextareaProcessor,

    oninput: EventHandler<String>,
}

#[derive(Clone, PartialEq, Default)]
pub struct LabeledTextareaProcessor {}

#[component]
pub fn LabeledTextarea(props: InputTextareaProps) -> Element {
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

            textarea {
                id:"{props.id}", name:"{props.name}", "type":"{props.input_type}", required: props.required,
                value: "{props.value}",
                class: "input  w-full min-h-[10rem] {input_class}",
                oninput: move |event| props.oninput.call(event.value())
            }

            if let Some(v) = props.violations {
                ErrorBlock {errors: v.errors}
            }
        }
    }
}
