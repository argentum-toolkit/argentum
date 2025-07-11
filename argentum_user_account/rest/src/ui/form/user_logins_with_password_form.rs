use crate::dto::schema::LoginWithPasswordSchema;
use crate::ui::form_data::UserLoginsWithPasswordFormProps;
use crate::ui::input::LoginWithPasswordSchemaInput;

use argentum_standard_ui::rsx::form::{LabeledInput, Submit};
use argentum_standard_ui::rsx::ErrorBlock;

use dioxus::prelude::*;

#[component]
pub fn UserLoginsWithPasswordForm(props: UserLoginsWithPasswordFormProps) -> Element {
    let mut form_data = props.form_data;

    rsx! {
        form {
            action:"#",
            class:"space-y-6",
            "novalidate": true,
            onsubmit: props.on_submit,

            ErrorBlock {errors: (form_data.errors)()}

            LoginWithPasswordSchemaInput {
                login_with_password_schema: (form_data.values)().into(),
                violations: (form_data.violations)(),
                oninput: move |event: LoginWithPasswordSchema| {
                    form_data.values.set(event);
                },
            }

            Submit {
                title: "Sign In".to_string(),
                disabled: (form_data.disabled)(),
            }
        }
    }
}
