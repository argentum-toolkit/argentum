use std::collections::BTreeMap;

use crate::dto::schema::GuestSendsMessageSchema;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_standard_ui::rsx::form::Captcha;
use argentum_standard_ui::rsx::form::CaptchaProcessor;
use argentum_standard_ui::rsx::form::LabeledInput;
use argentum_standard_ui::rsx::form::LabeledInputProcessor;
use argentum_standard_ui::rsx::form::LabeledTextarea;
use argentum_standard_ui::rsx::form::LabeledTextareaProcessor;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct GuestSendsMessageSchemaProps {
    pub value: GuestSendsMessageSchema,
    pub violations: Option<ViolationsDto>,
    pub ui_wrapper: String,
    pub title: Option<String>,

    processor: GuestSendsMessageSchemaInputProcessor,

    oninput: EventHandler<GuestSendsMessageSchema>,
}

#[derive(Clone, PartialEq, Default)]
pub struct GuestSendsMessageSchemaInputProcessor {
    pub name_processor: LabeledInputProcessor,
    pub email_processor: LabeledInputProcessor,
    pub subject_processor: LabeledInputProcessor,
    pub captcha_processor: CaptchaProcessor,
    pub message_processor: LabeledTextareaProcessor,
}

#[component]
pub fn GuestSendsMessageSchemaInput(props: GuestSendsMessageSchemaProps) -> Element {
    let label_class = match props.violations {
        Some(_) => "text-error",
        None => "",
    };

    let GuestSendsMessageSchemaProps {
        value,
        violations,
        processor,
        ui_wrapper,
        title,
        ..
    } = props.clone();

    let mut name_value = use_signal(|| value.name);
    let mut email_value = use_signal(|| value.email);
    let mut subject_value = use_signal(|| value.subject);
    let mut captcha_value = use_signal(|| value.captcha);
    let mut message_value = use_signal(|| value.message);

    let violation_items = match violations {
        Some(vv) => match vv.items {
            Some(ViolationItemDto::Object(items)) => items,
            _ => BTreeMap::new(),
        },
        None => BTreeMap::new(),
    };

    let name_violations = violation_items.get("name").cloned();
    let email_violations = violation_items.get("email").cloned();
    let subject_violations = violation_items.get("subject").cloned();
    let captcha_violations = violation_items.get("captcha").cloned();
    let message_violations = violation_items.get("message").cloned();

    let update = move || {
        props.oninput.call(GuestSendsMessageSchema::new(
            name_value(),
            email_value(),
            subject_value(),
            captcha_value(),
            message_value(),
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
                            LabeledInput {
            id: "name",
            name: "name",
            label: "Name",
            input_type: "text".to_string(),
            required: true,
                    value: name_value(),

            violations: name_violations,
            oninput: move |v: String| {
                name_value.set(v);
                update();
            },

            processor: processor.name_processor,
        }

                            LabeledInput {
            id: "email",
            name: "email",
            label: "Email",
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
            id: "subject",
            name: "subject",
            label: "Subject",
            input_type: "text".to_string(),
            required: true,
                    value: subject_value(),

            violations: subject_violations,
            oninput: move |v: String| {
                subject_value.set(v);
                update();
            },

            processor: processor.subject_processor,
        }

                            Captcha {
            id: "captcha",
            name: "captcha",
            label: "",
            required: false,
                value: captcha_value().unwrap_or_default(),
            violations: captcha_violations,

            oninput: move |v: String| {
                captcha_value.set(Some(v));
                update();
            },

            processor: processor.captcha_processor,
        }

                            LabeledTextarea {
            id: "message",
            name: "message",
            label: "Message",
            input_type: "text".to_string(),
            required: true,
                value: message_value(),

            violations: message_violations,
            oninput: move |v: String| {
                message_value.set(v);
                update();
            },

            processor: processor.message_processor,
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
                            LabeledInput {
            id: "name",
            name: "name",
            label: "Name",
            input_type: "text".to_string(),
            required: true,
                    value: name_value(),

            violations: name_violations,
            oninput: move |v: String| {
                name_value.set(v);
                update();
            },

            processor: processor.name_processor,
        }

                            LabeledInput {
            id: "email",
            name: "email",
            label: "Email",
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
            id: "subject",
            name: "subject",
            label: "Subject",
            input_type: "text".to_string(),
            required: true,
                    value: subject_value(),

            violations: subject_violations,
            oninput: move |v: String| {
                subject_value.set(v);
                update();
            },

            processor: processor.subject_processor,
        }

                            Captcha {
            id: "captcha",
            name: "captcha",
            label: "",
            required: false,
                value: captcha_value().unwrap_or_default(),
            violations: captcha_violations,

            oninput: move |v: String| {
                captcha_value.set(Some(v));
                update();
            },

            processor: processor.captcha_processor,
        }

                            LabeledTextarea {
            id: "message",
            name: "message",
            label: "Message",
            input_type: "text".to_string(),
            required: true,
                value: message_value(),

            violations: message_violations,
            oninput: move |v: String| {
                message_value.set(v);
                update();
            },

            processor: processor.message_processor,
        }

                        }
                    }
                }
    } else {
        rsx! {
                    LabeledInput {
            id: "name",
            name: "name",
            label: "Name",
            input_type: "text".to_string(),
            required: true,
                    value: name_value(),

            violations: name_violations,
            oninput: move |v: String| {
                name_value.set(v);
                update();
            },

            processor: processor.name_processor,
        }

                    LabeledInput {
            id: "email",
            name: "email",
            label: "Email",
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
            id: "subject",
            name: "subject",
            label: "Subject",
            input_type: "text".to_string(),
            required: true,
                    value: subject_value(),

            violations: subject_violations,
            oninput: move |v: String| {
                subject_value.set(v);
                update();
            },

            processor: processor.subject_processor,
        }

                    Captcha {
            id: "captcha",
            name: "captcha",
            label: "",
            required: false,
                value: captcha_value().unwrap_or_default(),
            violations: captcha_violations,

            oninput: move |v: String| {
                captcha_value.set(Some(v));
                update();
            },

            processor: processor.captcha_processor,
        }

                    LabeledTextarea {
            id: "message",
            name: "message",
            label: "Message",
            input_type: "text".to_string(),
            required: true,
                value: message_value(),

            violations: message_violations,
            oninput: move |v: String| {
                message_value.set(v);
                update();
            },

            processor: processor.message_processor,
        }

                }
    }
}
