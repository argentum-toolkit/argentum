use crate::rsx::ErrorBlock;
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
        Some(_) => "text-red-700 dark:text-red-500",
        None => "text-gray-900 dark:text-body-color-dark",
    };

    let input_bg = match props.violations {
        Some(_) => "bg-red-100 dark:bg-red-950 border-red-700 dark:border-red-500",
        None => "bg-gray-50 dark:bg-gray-700 border-gray-300 dark:border-gray-600",
    };

    rsx! {
        div { class: "mb-8 flex",
                label {
                    "htmlFor": "checkboxLabel",
                    class: "flex cursor-pointer select-none text-sm font-medium text-body-color {label_class}",
                    div { class: "relative",
                        input {
                            "type":"checkbox",
                            id: "checkboxLabel",
                            checked: "{props.value}",
                            class: "sr-only",
                            oninput: move |event| props.oninput.call(event.value() == *"true")
                        }
                        div { class: "{input_bg} box mr-3 flex h-5 w-5 items-center justify-center border-stroke dark:text-body-color-dark dark:shadow-two rounded-sm border text-base text-body-color outline-none transition-all duration-300 focus:border-primary dark:border-transparent dark:focus:border-primary dark:focus:shadow-none",
                            span {
                                class: "opacity-0",
                                svg {
                                    width:"11",
                                    height:"8",
                                    "viewBox":"0 0 11 8",
                                    fill:"none",
                                    xmlns:"http://www.w3.org/2000/svg",
                                    path {
                                        d:"M10.0915 0.951972L10.0867 0.946075L10.0813 0.940568C9.90076 0.753564 9.61034 0.753146 9.42927 0.939309L4.16201 6.22962L1.58507 3.63469C1.40401 3.44841 1.11351 3.44879 0.932892 3.63584C0.755703 3.81933 0.755703 4.10875 0.932892 4.29224L0.932878 4.29225L0.934851 4.29424L3.58046 6.95832C3.73676 7.11955 3.94983 7.2 4.1473 7.2C4.36196 7.2 4.55963 7.11773 4.71406 6.9584L10.0468 1.60234C10.2436 1.4199 10.2421 1.1339 10.0915 0.951972ZM4.2327 6.30081L4.2317 6.2998C4.23206 6.30015 4.23237 6.30049 4.23269 6.30082L4.2327 6.30081Z",
                                        fill:"#3056E3",
                                        stroke:"#3056E3",
                                        "strokeWidth":"0.4",
                                    }
                                }
                            }
                        }
                    }
                    span {
                        "{props.label}"
                    }
                }

                if let Some(v) = props.violations {
                    ErrorBlock {errors: v.errors}
                }
            }
    }
}
