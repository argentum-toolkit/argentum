use crate::dto::schema::RequestRestoreTokenSchema;
use crate::ui::input::RequestRestoreTokenSchemaInput;
use crate::ui::input::RequestRestoreTokenSchemaInputProcessor;

use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct RequestRestoreTokenSchemaOptionalProps {
    pub optional_title: Option<String>,
    pub optional_style: String,
    pub value: Option<RequestRestoreTokenSchema>,
    pub violations: Option<ViolationsDto>,
    pub ui_wrapper: String,
    pub title: Option<String>,

    processor: RequestRestoreTokenSchemaInputProcessor,

    oninput: EventHandler<Option<RequestRestoreTokenSchema>>,
}

#[component]
pub fn RequestRestoreTokenSchemaOptionalInput(
    props: RequestRestoreTokenSchemaOptionalProps,
) -> Element {
    let RequestRestoreTokenSchemaOptionalProps {
        violations,
        processor,
        oninput,
        optional_title,
        optional_style,
        ui_wrapper,
        title,
        ..
    } = props.clone();

    let mut old_value = use_signal(|| props.value.clone());
    let mut optional_value = use_signal(|| props.value.clone());

    let update = move || oninput.call(optional_value());

    rsx! {
        fieldset {
            label {
                class: "label",
                input {
                    "type":"checkbox",
                    checked: (optional_value().is_none() && optional_style == "inverted") || (optional_value().is_some() && optional_style == "normal"),
                    class: "checkbox",
                    oninput: move |event| {
                        if (optional_style == "inverted" && "true" != event.value()) || (optional_style != "inverted" && "true" == event.value())  {
                            let v = match old_value() {
                                Some(v) => Some(v),
                                None => Some(Default::default()),
                            };
                            optional_value.set(v);
                        } else {
                            old_value.set(optional_value());
                            optional_value.set(None);
                        }
                        update();
                    }
                }

                match optional_title {
                    Some(t) => rsx! {"{t}"},
                    None => rsx! { "Empty/Not Empty" },
                }
            }

            match optional_value() {
                Some(value) => rsx! {
                    div {
                        RequestRestoreTokenSchemaInput {
                            value,
                            processor,
                            violations,
                            title,
                            ui_wrapper: ui_wrapper.clone(),
                            oninput: move |v: RequestRestoreTokenSchema| {
                                optional_value.set(Some(v));
                                update();
                            },

                        }
                    }
                },
                None => rsx! {},
            }
        }
    }
}
