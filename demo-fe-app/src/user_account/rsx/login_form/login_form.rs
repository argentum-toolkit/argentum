use argentum_standard_ui::rsx::form::{LabeledInput, Submit};
use argentum_standard_ui::rsx::ErrorBlock;

use argentum_user_account_rest::ui::form_data::UserLoginsWithPasswordProps;
use dioxus::prelude::*;

#[component]
pub fn LoginWithPasswordForm(props: UserLoginsWithPasswordProps) -> Element {
    #[cfg(not(feature = "web"))]
    let (on_submit, mut form_data) = {
        use argentum_user_account_rest::ui::form_data::UserLoginsWithPasswordFormData;

        (move |_| {}, UserLoginsWithPasswordFormData::new())
    };

    #[cfg(feature = "web")]
    let (on_submit, mut form_data) = {
        use super::web_boilerplate::create_form_boilerplate;

        create_form_boilerplate(props)
    };

    rsx! {
        form {
            onsubmit: on_submit,
            class:"space-y-6", action:"#", method:"POST",
            "novalidate": true,
            ErrorBlock {errors: (form_data.errors)()}

            LabeledInput {
                //todo: id should be longer
                id: "email".to_string(),
                name: "email".to_string(),
                label: "Email address".to_string(),
                input_type: "email".to_string(),
                value: (form_data.values).email,
                violations: (form_data.violations.email)(),
                oninput: move |event: String| (form_data.values.email).set(event),
            },

            LabeledInput {
                id: "password".to_string(),
                name: "password".to_string(),
                label: "Password".to_string(),
                input_type: "password".to_string(),
                value: form_data.values.password,
                violations: (form_data.violations.password)(),
                oninput: move |event: String| (form_data.values.password).set(event),
            },

            Submit {
                title: "Sign In".to_string(),
                disabled: (form_data.disabled)(),
            }
        }
    }
}
