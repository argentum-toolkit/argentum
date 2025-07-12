use crate::dto::schema::RegistrationWithPasswordSchema;
use crate::ui::form_data::UserRegistersWithPasswordFormProps;
use crate::ui::input::RegistrationWithPasswordSchemaInput;

use argentum_standard_ui::rsx::form::{LabeledInput, Submit};
use argentum_standard_ui::rsx::ErrorBlock;

use dioxus::prelude::*;

#[component]
pub fn UserRegistersWithPasswordForm(props: UserRegistersWithPasswordFormProps) -> Element {
    let mut form_data = props.form_data;

    rsx! {
        form {
            action:"#",
            class:"space-y-6",
            "novalidate": true,
            onsubmit: props.on_submit,

            ErrorBlock {errors: (form_data.errors)()}

            RegistrationWithPasswordSchemaInput {
                registration_with_password_schema: (form_data.values)().into(),
                violations: (form_data.violations)(),
                oninput: move |event: RegistrationWithPasswordSchema| {
                    form_data.values.set(event);
                },
            }

            Submit {
                title: "Submit".to_string(),
                disabled: (form_data.disabled)(),
            }
        }
    }
}
