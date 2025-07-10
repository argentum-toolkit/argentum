use std::collections::BTreeMap;

use crate::dto::schema::RequestRestoreTokenSchema;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_standard_ui::rsx::form::LabeledInput;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct RequestRestoreTokenSchemaProps {
    pub request_restore_token_schema: RequestRestoreTokenSchema,
    pub violations: Option<ViolationsDto>,

    oninput: EventHandler<RequestRestoreTokenSchema>,
}

#[component]
pub fn RequestRestoreTokenSchemaInput(props: RequestRestoreTokenSchemaProps) -> Element {
    let RequestRestoreTokenSchemaProps {
        request_restore_token_schema,
        violations,
        ..
    } = props.clone();

    let mut email_value = use_signal(|| request_restore_token_schema.email.clone());

    let violation_items = match violations {
        Some(vv) => match vv.items {
            Some(ViolationItemDto::Object(items)) => items,
            _ => BTreeMap::new(),
        },
        None => BTreeMap::new(),
    };

    let email_violations = violation_items.get("email").cloned();

    let update = move || {
        props
            .oninput
            .call(RequestRestoreTokenSchema::new(email_value()))
    };

    rsx! {
            LabeledInput {
        id: "unknown",
        name: "unknown",
        label: "",
        input_type: "text".to_string(),
            value: email_value,

        violations: email_violations,
        oninput: move |event: String| {
            email_value.set(event);
            update();
        },
    }

        }
}
