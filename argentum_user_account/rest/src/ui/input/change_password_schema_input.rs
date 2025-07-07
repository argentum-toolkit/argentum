use std::collections::BTreeMap;

use crate::dto::schema::ChangePasswordSchema;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_standard_ui::rsx::form::LabeledInput;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ChangePasswordSchemaProps {
    pub change_password_schema: ChangePasswordSchema,
    pub violations: Option<ViolationsDto>,

    oninput: EventHandler<ChangePasswordSchema>,
}

#[component]
pub fn ChangePasswordSchemaInput(props: ChangePasswordSchemaProps) -> Element {
    let ChangePasswordSchemaProps {
        change_password_schema,
        violations,
        ..
    } = props.clone();

    let mut password_value = use_signal(|| change_password_schema.password.clone());
    let mut token_value = use_signal(|| change_password_schema.token.clone());

    let violation_items = match violations {
        Some(vv) => match vv.items {
            Some(ViolationItemDto::Object(items)) => items,
            _ => BTreeMap::new(),
        },
        None => BTreeMap::new(),
    };

    let password_violations = violation_items.get("password").cloned();
    let token_violations = violation_items.get("token").cloned();

    let update = move || {
        props
            .oninput
            .call(ChangePasswordSchema::new(password_value(), token_value()))
    };

    rsx! {
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

            LabeledInput {
        id: "unknown",
        name: "unknown",
        label: "unknown",
        input_type: "text".to_string(),
            value: token_value,

        violations: token_violations,
        oninput: move |event: String| {
            token_value.set(event);
            update();
        },
    }

        }
}
