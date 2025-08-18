use crate::rsx::component::ErrorBlock;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
#[props(borrowed)]
pub struct InputTextProps {
    pub value: String,
    pub id: String,
    pub name: String,
    pub input_type: String,
    pub label: String,
    pub violations: Option<ViolationsDto>,

    oninput: EventHandler<String>,
}

#[component]
pub fn LabeledInput(props: InputTextProps) -> Element {
    let label_class = match props.violations {
        Some(_) => "text-error",
        None => "",
    };

    let input_class = match props.violations {
        Some(_) => "input-error",
        None => "",
    };

    rsx! {
        fieldset {
            class: "fieldset",
            label {
                "for":"{props.id}",
                class: "fieldset-legend {label_class}",
                "{props.label}"
            }

            input {
                id:"{props.id}", name:"{props.name}", "type":"{props.input_type}", autocomplete:"email", required:true,
                value: "{props.value}",
                class: "input {input_class}",
                oninput: move |event| props.oninput.call(event.value())
            }

            if let Some(v) = props.violations {
                ErrorBlock {errors: v.errors}
            }
        }
    }
}
