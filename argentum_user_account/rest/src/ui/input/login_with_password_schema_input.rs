use std::collections::BTreeMap;

use crate::dto::schema::LoginWithPasswordSchema;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_standard_ui::rsx::form::LabeledInput;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct LoginWithPasswordSchemaProps {
    pub login_with_password_schema: LoginWithPasswordSchema,
    pub violations: Option<ViolationsDto>,

    oninput: EventHandler<LoginWithPasswordSchema>,
}

#[component]
pub fn LoginWithPasswordSchemaInput(props: LoginWithPasswordSchemaProps) -> Element {
    let LoginWithPasswordSchemaProps {
        login_with_password_schema,
        violations,
        ..
    } = props.clone();

    let mut email_value = use_signal(|| login_with_password_schema.email.clone());
    let mut password_value = use_signal(|| login_with_password_schema.password.clone());

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
        label: "unknown",
        input_type: "text".to_string(),
            value: email_value,

        violations: email_violations,
        oninput: move |event: String| {
            email_value.set(event);
            update();
        },
    }

            LabeledInput {
        id: "unknown",
        name: "unknown",
        label: "unknown",
        input_type: "text".to_string(),
            value: password_value,

        violations: password_violations,
        oninput: move |event: String| {
            password_value.set(event);
            update();
        },
    }

        }
}
