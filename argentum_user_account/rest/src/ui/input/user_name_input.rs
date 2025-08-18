use std::collections::BTreeMap;

use crate::dto::schema::UserName;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_standard_ui::rsx::form::LabeledInput;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct UserNameProps {
    pub value: UserName,
    pub violations: Option<ViolationsDto>,

    oninput: EventHandler<UserName>,
}

#[component]
pub fn UserNameInput(props: UserNameProps) -> Element {
    let UserNameProps {
        value, violations, ..
    } = props.clone();

    let mut first_value = use_signal(|| value.first);
    let mut last_value = use_signal(|| value.last);
    let mut additional_value = use_signal(|| value.additional);

    let violation_items = match violations {
        Some(vv) => match vv.items {
            Some(ViolationItemDto::Object(items)) => items,
            _ => BTreeMap::new(),
        },
        None => BTreeMap::new(),
    };

    let first_violations = violation_items.get("first").cloned();
    let last_violations = violation_items.get("last").cloned();
    let additional_violations = violation_items.get("additional").cloned();

    let update = move || {
        props.oninput.call(UserName::new(
            first_value(),
            last_value(),
            additional_value(),
        ))
    };

    rsx! {
            LabeledInput {
        id: "user_first_name",
        name: "user_first_name",
        label: "First Name",
        input_type: "text".to_string(),
            value: first_value,

        violations: first_violations,
        oninput: move |event: String| {
            first_value.set(event);
            update();
        },
    }

            LabeledInput {
        id: "user_last_name",
        name: "user_last_name",
        label: "Last Name",
        input_type: "text".to_string(),
            value: last_value().unwrap_or("".to_string()),

        violations: last_violations,
        oninput: move |event: String| {
            last_value.set(Some(event));
            update();
        },
    }

            LabeledInput {
        id: "user_additional",
        name: "user_additional",
        label: "Additional",
        input_type: "text".to_string(),
            value: additional_value().unwrap_or("".to_string()),

        violations: additional_violations,
        oninput: move |event: String| {
            additional_value.set(Some(event));
            update();
        },
    }

        }
}
