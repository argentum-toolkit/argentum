use std::collections::BTreeMap;

use crate::dto::schema::RegistrationWithPasswordSchema;

use crate::dto::schema::UserName;
use crate::ui::input::UserNameInput;
use crate::ui::input::UserNameInputProcessor;
use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_standard_ui::rsx::form::LabeledCheckbox;
use argentum_standard_ui::rsx::form::LabeledCheckboxProcessor;
use argentum_standard_ui::rsx::form::LabeledInput;
use argentum_standard_ui::rsx::form::LabeledInputProcessor;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct RegistrationWithPasswordSchemaProps {
    pub value: RegistrationWithPasswordSchema,
    pub violations: Option<ViolationsDto>,
    pub ui_wrapper: String,
    pub title: Option<String>,

    processor: RegistrationWithPasswordSchemaInputProcessor,

    oninput: EventHandler<RegistrationWithPasswordSchema>,
}

#[derive(Clone, PartialEq, Default)]
pub struct RegistrationWithPasswordSchemaInputProcessor {
    pub name_processor: UserNameInputProcessor,
    pub email_processor: LabeledInputProcessor,
    pub password_processor: LabeledInputProcessor,
    pub terms_processor: LabeledCheckboxProcessor,
}

#[component]
pub fn RegistrationWithPasswordSchemaInput(props: RegistrationWithPasswordSchemaProps) -> Element {
    let label_class = match props.violations {
        Some(_) => "text-error",
        None => "",
    };

    let RegistrationWithPasswordSchemaProps {
        value,
        violations,
        processor,
        ui_wrapper,
        title,
        ..
    } = props.clone();

    let mut name_value = use_signal(|| value.name);
    let mut email_value = use_signal(|| value.email);
    let mut password_value = use_signal(|| value.password);
    let mut terms_value = use_signal(|| value.terms);

    let violation_items = match violations {
        Some(vv) => match vv.items {
            Some(ViolationItemDto::Object(items)) => items,
            _ => BTreeMap::new(),
        },
        None => BTreeMap::new(),
    };

    let name_violations = violation_items.get("name").cloned();
    let email_violations = violation_items.get("email").cloned();
    let password_violations = violation_items.get("password").cloned();
    let terms_violations = violation_items.get("terms").cloned();

    let update = move || {
        props.oninput.call(RegistrationWithPasswordSchema::new(
            name_value(),
            email_value(),
            password_value(),
            terms_value(),
        ))
    };

    if ui_wrapper.eq("flex") {
        rsx! {
                    fieldset {
                        if let Some(t) = title {
                            legend { class: "fieldset-legend {label_class}", "{t}"}
                        }
                        div {
                            class: "flex gap-2",

                        UserNameInput {
                            value: name_value(),
                            violations: name_violations,
                            ui_wrapper: "none".to_string(),
                            oninput: move |v: UserName| {
                                name_value.set(v);
                                update();
                            },
                            processor: processor.name_processor,
                        }

                            LabeledInput {
            id: "email",
            name: "email",
            label: "Email address",
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

                            LabeledInput {
            id: "password",
            name: "password",
            label: "Password",
            input_type: "text".to_string(),
            required: true,
                value: password_value(),

            violations: password_violations,
            oninput: move |v: String| {
                password_value.set(v);
                update();
            },

            processor: processor.password_processor,
        }

                            LabeledCheckbox {
            id: "terms",
            name: "terms",
            label: "By creating account means you agree to the Terms and Conditions, and our Privacy Policy",
                value: terms_value(),
            violations: terms_violations,

            oninput: move |v: bool| {
                terms_value.set(v);
                update();
            },

            processor: processor.terms_processor,
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

                        UserNameInput {
                            value: name_value(),
                            violations: name_violations,
                            ui_wrapper: "none".to_string(),
                            oninput: move |v: UserName| {
                                name_value.set(v);
                                update();
                            },
                            processor: processor.name_processor,
                        }

                            LabeledInput {
            id: "email",
            name: "email",
            label: "Email address",
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

                            LabeledInput {
            id: "password",
            name: "password",
            label: "Password",
            input_type: "text".to_string(),
            required: true,
                value: password_value(),

            violations: password_violations,
            oninput: move |v: String| {
                password_value.set(v);
                update();
            },

            processor: processor.password_processor,
        }

                            LabeledCheckbox {
            id: "terms",
            name: "terms",
            label: "By creating account means you agree to the Terms and Conditions, and our Privacy Policy",
                value: terms_value(),
            violations: terms_violations,

            oninput: move |v: bool| {
                terms_value.set(v);
                update();
            },

            processor: processor.terms_processor,
        }

                        }
                    }
                }
    } else {
        rsx! {

                        UserNameInput {
                            value: name_value(),
                            violations: name_violations,
                            ui_wrapper: "none".to_string(),
                            oninput: move |v: UserName| {
                                name_value.set(v);
                                update();
                            },
                            processor: processor.name_processor,
                        }

                    LabeledInput {
            id: "email",
            name: "email",
            label: "Email address",
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

                    LabeledInput {
            id: "password",
            name: "password",
            label: "Password",
            input_type: "text".to_string(),
            required: true,
                value: password_value(),

            violations: password_violations,
            oninput: move |v: String| {
                password_value.set(v);
                update();
            },

            processor: processor.password_processor,
        }

                    LabeledCheckbox {
            id: "terms",
            name: "terms",
            label: "By creating account means you agree to the Terms and Conditions, and our Privacy Policy",
                value: terms_value(),
            violations: terms_violations,

            oninput: move |v: bool| {
                terms_value.set(v);
                update();
            },

            processor: processor.terms_processor,
        }

                }
    }
}
