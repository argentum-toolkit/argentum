use std::pin::Pin;
use std::sync::Arc;

use crate::rsx::component::ErrorBlock;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use dioxus::prelude::*;

enum AutocompleteState {
    Hidden,
    Loading,
    Loaded(Vec<(uuid::Uuid, String)>),
}
#[derive(Clone, PartialEq, Props)]
#[props(borrowed)]
pub struct AutocompleteUuidProps {
    pub value: String,
    pub id: String,
    pub name: String,
    pub input_type: String,
    pub required: bool,
    pub label: String,
    pub violations: Option<ViolationsDto>,

    processor: AutocompleteUuidProcessor,

    oninput: EventHandler<uuid::Uuid>,
}

#[derive(Clone)]
pub struct AutocompleteUuidProcessor {
    pub fetch_data: Arc<
        dyn Fn(&str) -> Pin<Box<dyn Future<Output = Vec<(uuid::Uuid, String)>> + 'static>>
            + 'static,
    >,
}

impl Default for AutocompleteUuidProcessor {
    fn default() -> Self {
        Self {
            fetch_data: Arc::new(|_query: &str| Box::pin(async move { vec![] })),
        }
    }
}

impl PartialEq for AutocompleteUuidProcessor {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[component]
pub fn AutocompleteUuid(props: AutocompleteUuidProps) -> Element {
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

    let mut state = use_signal(|| AutocompleteState::Hidden);
    let mut string_value = use_signal(|| "".to_string());

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
                autocomplete: "off",
                value: string_value(),
                class: "input  w-full {input_class}",
                oninput: move |event| {
                    state.set(AutocompleteState::Loading);

                    let part = event.value();
                    let cloned_part = part.clone();

                    let fetch = props.processor.fetch_data.clone();

                    spawn(async move {
                        let v = fetch(&cloned_part).await;
                        string_value.set(cloned_part);
                        state.set(AutocompleteState::Loaded(v));

                    });
                }
            }

            match *state.read() {
                AutocompleteState::Loading => rsx!{ div { "Loading..." }},
                AutocompleteState::Loaded(ref values) => rsx!{div {
                    div {
                        {values.iter().map(|(id, title)| {
                            let deref_id = *id;
                            let cloned_title = title.clone();
                            rsx! {
                                div {
                                    onclick: move |_| {
                                        string_value.set(cloned_title.clone());

                                        props.oninput.call(deref_id);
                                        state.set(AutocompleteState::Hidden);
                                    },
                                    "{title}",
                                }
                            }
                        })}
                    }
                }},
                _ => rsx!{ },
            }


            if let Some(v) = props.violations {
                ErrorBlock {errors: v.errors}
            }
        }
    }
}
