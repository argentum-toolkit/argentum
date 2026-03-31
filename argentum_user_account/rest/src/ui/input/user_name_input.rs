use std::collections::BTreeMap;

use crate::dto::schema::UserName;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_standard_ui::rsx::form::LabeledInput;
use argentum_standard_ui::rsx::form::LabeledInputProcessor;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct UserNameProps {
    pub value: UserName,
    pub violations: Option<ViolationsDto>,

    processor: UserNameInputProcessor,

    oninput: EventHandler<UserName>,
}

#[derive(Clone, PartialEq, Default)]
pub struct UserNameInputProcessor {
    pub first_processor: LabeledInputProcessor,
    pub last_processor: LabeledInputProcessor,
    pub additional_processor: LabeledInputProcessor,
}

#[component]
pub fn UserNameInput(props: UserNameProps) -> Element {
    let UserNameProps {
        value,
        violations,
        processor,
        ..
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
        required: true,
            value: first_value,

        violations: first_violations,
        oninput: move |v: String| {
            first_value.set(v);
            update();
        },

        processor: processor.first_processor,
    }

            LabeledInput {
        id: "user_last_name",
        name: "user_last_name",
        label: "Last Name",
        input_type: "text".to_string(),
        required: false,
            value: last_value().unwrap_or("".to_string()),

        violations: last_violations,
        oninput: move |v: String| {
            last_value.set(Some(v));
            update();
        },

        processor: processor.last_processor,
    }

            LabeledInput {
        id: "user_additional_name",
        name: "user_additional_name",
        label: "Additional name",
        input_type: "text".to_string(),
        required: false,
            value: additional_value().unwrap_or("".to_string()),

        violations: additional_violations,
        oninput: move |v: String| {
            additional_value.set(Some(v));
            update();
        },

        processor: processor.additional_processor,
    }

        }
}
