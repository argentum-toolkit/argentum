use std::collections::BTreeMap;

use crate::dto::schema::RegistrationWithPasswordSchema;

use crate::dto::schema::UserName;
use crate::ui::input::UserNameInput;
use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};
use argentum_standard_ui::rsx::form::LabeledCheckbox;
use argentum_standard_ui::rsx::form::LabeledInput;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct RegistrationWithPasswordSchemaProps {
    pub value: RegistrationWithPasswordSchema,
    pub violations: Option<ViolationsDto>,

    oninput: EventHandler<RegistrationWithPasswordSchema>,
}

#[component]
pub fn RegistrationWithPasswordSchemaInput(props: RegistrationWithPasswordSchemaProps) -> Element {
    let RegistrationWithPasswordSchemaProps {
        value, violations, ..
    } = props.clone();

    let mut email_value = use_signal(|| value.email);
    let mut name_value = use_signal(|| value.name);
    let mut password_value = use_signal(|| value.password);
    let mut terms_value = use_signal(|| value.terms);

    let violation_items = match violations {
        Some(vv) => match vv.items {
            Some(ViolationItemDto::Object(items)) => items,
            _ => BTreeMap::new(),
        },
        None => BTreeMap::new(),
    };

    let email_violations = violation_items.get("email").cloned();
    let name_violations = violation_items.get("name").cloned();
    let password_violations = violation_items.get("password").cloned();
    let terms_violations = violation_items.get("terms").cloned();

    let update = move || {
        props.oninput.call(RegistrationWithPasswordSchema::new(
            email_value(),
            name_value(),
            password_value(),
            terms_value(),
        ))
    };

    rsx! {
            LabeledInput {
        id: "email",
        name: "email",
        label: "Email address",
        input_type: "text".to_string(),
            value: email_value,

        violations: email_violations,
        oninput: move |event: String| {
            email_value.set(event);
            update();
        },
    }

            UserNameInput {
        value: name_value(),
        violations: name_violations,
        oninput: move |event: UserName| {
            name_value.set(event);
            update();
        }
    }

            LabeledInput {
        id: "password",
        name: "password",
        label: "Password",
        input_type: "text".to_string(),
            value: password_value,

        violations: password_violations,
        oninput: move |event: String| {
            password_value.set(event);
            update();
        },
    }

            LabeledCheckbox {
        id: "terms",
        name: "terms",
        label: "By creating account means you agree to the Terms and Conditions, and our Privacy Policy",
            value: terms_value(),
        violations: terms_violations,
        oninput: move |event: bool| {
            terms_value.set(event);
            update();
        }
    }

        }
}
