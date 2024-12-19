use crate::rsx::ErrorBlock;
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
        Some(_) => "text-red-700 dark:text-red-500",
        None => "text-gray-900 dark:text-body-color-dark",
    };

    let input_bg = match props.violations {
        Some(_) => "bg-red-100 dark:bg-red-950 border-red-700 dark:border-red-500",
        None => "bg-gray-50 dark:bg-gray-700 border-gray-300 dark:border-gray-600",
    };

    rsx! {
        div {
            label { "for":"{props.id}",
                class: "block text-sm font-medium leading-6 {label_class}",
                "{props.label}"
            }

            div { class:"mt-2",
                input {
                    id:"{props.id}", name:"{props.name}", "type":"{props.input_type}", autocomplete:"email", required:true,
                    value: "{props.value}",
                    class: "{input_bg} border-stroke dark:text-body-color-dark dark:shadow-two w-full rounded-sm border px-6 py-3 text-base text-body-color outline-none transition-all duration-300 focus:border-primary dark:border-transparent dark:focus:border-primary dark:focus:shadow-none",
                    oninput: move |event| props.oninput.call(event.value())
                }

                if let Some(v) = props.violations {
                    ErrorBlock {errors: v.errors}
                }
            }
        }
    }
}
