use std::collections::BTreeMap;

use crate::dto::schema::UserName;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_standard_ui::rsx::form::LabeledInput;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct UserNameProps {
    pub user_name: UserName,
    pub violations: Option<ViolationsDto>,

    oninput: EventHandler<UserName>,
}

#[component]
pub fn UserNameInput(props: UserNameProps) -> Element {
    let UserNameProps {
        user_name,
        violations,
        ..
    } = props.clone();

    let mut first_value = use_signal(|| user_name.first.clone());
    let mut last_value = use_signal(|| user_name.last.clone());
    let mut patronymic_value = use_signal(|| user_name.patronymic.clone());

    let violation_items = match violations {
        Some(vv) => match vv.items {
            Some(ViolationItemDto::Object(items)) => items,
            _ => BTreeMap::new(),
        },
        None => BTreeMap::new(),
    };

    let first_violations = violation_items.get("first").cloned();
    let last_violations = violation_items.get("last").cloned();
    let patronymic_violations = violation_items.get("patronymic").cloned();

    let update = move || {
        props.oninput.call(UserName::new(
            first_value(),
            last_value(),
            patronymic_value(),
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
        label: "First Name",
        input_type: "text".to_string(),
            value: last_value().unwrap_or("".to_string()),

        violations: last_violations,
        oninput: move |event: String| {
            last_value.set(Some(event));
            update();
        },
    }

            LabeledInput {
        id: "user_patronymic",
        name: "user_patronymic",
        label: "Patronymic",
        input_type: "text".to_string(),
            value: patronymic_value().unwrap_or("".to_string()),

        violations: patronymic_violations,
        oninput: move |event: String| {
            patronymic_value.set(Some(event));
            update();
        },
    }

        }
}
