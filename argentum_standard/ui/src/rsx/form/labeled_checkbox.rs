use crate::rsx::component::ErrorBlock;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
#[props(borrowed)]
pub struct CheckboxProps {
    pub value: bool,
    pub id: String,
    pub name: String,
    pub label: String,
    pub violations: Option<ViolationsDto>,

    oninput: EventHandler<bool>,
}

#[component]
pub fn LabeledCheckbox(props: CheckboxProps) -> Element {
    let label_class = match props.violations {
        Some(_) => "text-error",
        None => "",
    };

    let input_class = match props.violations {
        Some(_) => "text-error",
        None => "",
    };

    rsx! {
        fieldset { class: "fieldset",
            label {
                "htmlFor": "checkboxLabel",
                class: "label {label_class}",
                input {
                    "type":"checkbox",
                    id: "checkboxLabel",
                    checked: "{props.value}",
                    class: "checkbox {input_class}",
                    oninput: move |event| props.oninput.call(event.value() == *"true")
                }
                {props.label}
            }

            if let Some(v) = props.violations {
                ErrorBlock {errors: v.errors}
            }
        }
    }
}
