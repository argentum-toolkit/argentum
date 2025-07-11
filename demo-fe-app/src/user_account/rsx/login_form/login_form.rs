use argentum_standard_ui::rsx::form::{LabeledInput, Submit};
use argentum_standard_ui::rsx::ErrorBlock;

use argentum_user_account_rest::dto::schema::LoginWithPasswordSchema;
use argentum_user_account_rest::ui::form_data::UserLoginsWithPasswordFormProps;
use argentum_user_account_rest::ui::input::LoginWithPasswordSchemaInput;
use dioxus::prelude::*;

#[component]
pub fn LoginWithPasswordForm(props: UserLoginsWithPasswordFormProps) -> Element {
    let mut form_data = props.form_data;

    rsx! {
        form {
            onsubmit: props.on_submit,
            action:"#",
            class:"space-y-6",
            "novalidate": true,
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
