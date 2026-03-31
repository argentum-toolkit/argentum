use std::collections::BTreeMap;

use crate::dto::schema::LoginWithPasswordSchema;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_standard_ui::rsx::form::LabeledInput;
use argentum_standard_ui::rsx::form::LabeledInputProcessor;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct LoginWithPasswordSchemaProps {
    pub value: LoginWithPasswordSchema,
    pub violations: Option<ViolationsDto>,

    processor: LoginWithPasswordSchemaInputProcessor,

    oninput: EventHandler<LoginWithPasswordSchema>,
}

#[derive(Clone, PartialEq, Default)]
pub struct LoginWithPasswordSchemaInputProcessor {
    pub email_processor: LabeledInputProcessor,
    pub password_processor: LabeledInputProcessor,
}

#[component]
pub fn LoginWithPasswordSchemaInput(props: LoginWithPasswordSchemaProps) -> Element {
    let LoginWithPasswordSchemaProps {
        value,
        violations,
        processor,
        ..
    } = props.clone();

    let mut email_value = use_signal(|| value.email);
    let mut password_value = use_signal(|| value.password);

    let violation_items = match violations {
        Some(vv) => match vv.items {
            Some(ViolationItemDto::Object(items)) => items,
            _ => BTreeMap::new(),
        },
        None => BTreeMap::new(),
    };

    let email_violations = violation_items.get("email").cloned();
    let password_violations = violation_items.get("password").cloned();

    let update = move || {
        props.oninput.call(LoginWithPasswordSchema::new(
            email_value(),
            password_value(),
        ))
    };

    rsx! {
            LabeledInput {
        id: "unknown",
        name: "unknown",
        label: "Email",
        input_type: "text".to_string(),
        required: true,
            value: email_value,

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
            value: password_value,

        violations: password_violations,
        oninput: move |v: String| {
            password_value.set(v);
            update();
        },

        processor: processor.password_processor,
    }

        }
}
