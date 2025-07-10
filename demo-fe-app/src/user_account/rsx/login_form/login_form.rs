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
            class:"space-y-6", action:"#", method:"POST",
            "novalidate": true,
            ErrorBlock {errors: (form_data.errors)()}

            LoginWithPasswordSchemaInput {
                login_with_password_schema: LoginWithPasswordSchema {
                    email: (form_data.values.email)(),
                    password: (form_data.values.password)(),
                },//TODO implement into
                violations: form_data.violations,
                oninput: move |event: LoginWithPasswordSchema| {
                    form_data.values.email.set(event.email);
                    form_data.values.password.set(event.password);
                },
            }

            Submit {
                title: "Sign In".to_string(),
                disabled: (form_data.disabled)(),
            }
        }
    }
}
