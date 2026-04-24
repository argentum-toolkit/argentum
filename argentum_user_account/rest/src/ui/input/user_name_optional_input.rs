use crate::dto::schema::UserName;
use crate::ui::input::UserNameInput;
use crate::ui::input::UserNameInputProcessor;

use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct UserNameOptionalProps {
    pub optional_title: Option<String>,
    pub optional_style: String,
    pub value: Option<UserName>,
    pub violations: Option<ViolationsDto>,
    pub ui_wrapper: String,
    pub title: Option<String>,

    processor: UserNameInputProcessor,

    oninput: EventHandler<Option<UserName>>,
}

#[component]
pub fn UserNameOptionalInput(props: UserNameOptionalProps) -> Element {
    let UserNameOptionalProps {
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
                        UserNameInput {
                            value,
                            processor,
                            violations,
                            title,
                            ui_wrapper: ui_wrapper.clone(),
                            oninput: move |v: UserName| {
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
