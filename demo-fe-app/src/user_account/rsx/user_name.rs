use std::collections::BTreeMap;

use crate::standard::rsx::LabeledInput;
use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_user_account_rest::dto::schema::UserName;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[derive(PartialEq, Props, Clone)]
pub struct UserNameProps {
    pub user_name: UserName,
    pub violations: Option<ViolationsDto>,

    oninput: EventHandler<UserName>,
}

#[component]
pub fn UserNameComponent(props: UserNameProps) -> Element {
    let UserNameProps {
        user_name,
        violations,
        ..
    } = props.clone();

    let mut first = use_signal(|| user_name.first.clone());
    let mut last = use_signal(|| user_name.last.unwrap_or("".to_string()));

    let violation_items = match violations {
        Some(vv) => match vv.items {
            Some(ViolationItemDto::Object(items)) => items,
            _ => BTreeMap::new(),
        },
        None => BTreeMap::new(),
    };

    let first_violations = violation_items.get("first").cloned();
    let last_violations = violation_items.get("last").cloned();

    let update = move || {
        props
            .oninput
            .call(UserName::new(first(), Some(last()), None))
    };

    rsx! {
        LabeledInput {
            id: "first_name".to_string(),
            name: "first_name".to_string(),
            label: "First Name".to_string(),
            input_type: "text".to_string(),
            value: first,
            violations: first_violations,
            oninput: move |event: String| {
                first.set(event);
                update();
            },
        },

        LabeledInput {
            id: "last_name".to_string(),
            name: "last_name".to_string(),
            label: "Last Name".to_string(),
            input_type: "text".to_string(),
            value: last,
            violations: last_violations,
            oninput: move |event: String| {
                last.set(event);
                update();
            },
        },
    }
}
