use crate::dto::schema::RequestRestoreTokenSchema;
use crate::ui::form_data::AnonymousRequestsRestoreTokenFormProps;
use crate::ui::input::RequestRestoreTokenSchemaInput;

use argentum_standard_ui::rsx::form::{LabeledInput, Submit};
use argentum_standard_ui::rsx::ErrorBlock;

use dioxus::prelude::*;

#[component]
pub fn AnonymousRequestsRestoreTokenForm(props: AnonymousRequestsRestoreTokenFormProps) -> Element {
    let mut form_data = props.form_data;

    rsx! {
        form {
            action:"#",
            class:"space-y-6",
            "novalidate": true,
            onsubmit: props.on_submit,

            ErrorBlock {errors: (form_data.errors)()}

            RequestRestoreTokenSchemaInput {
                request_restore_token_schema: (form_data.values)().into(),
                violations: (form_data.violations)(),
                oninput: move |event: RequestRestoreTokenSchema| {
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
