use crate::dto::schema::ChangePasswordSchema;
use crate::ui::form_processor::AnonymousWithTokenChangesPasswordFormProcessor;
use crate::ui::input::ChangePasswordSchemaInput;
use std::sync::Arc;

use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use argentum_standard_ui::rsx::form::Submit;
use argentum_standard_ui::rsx::ErrorBlock;

use dioxus::prelude::*;

#[component]
pub fn AnonymousWithTokenChangesPasswordForm(
    processor: Arc<AnonymousWithTokenChangesPasswordFormProcessor>,
    auth_token: String,
) -> Element {
    let mut values: Signal<ChangePasswordSchema> = use_signal(|| Default::default());
    let violations: Signal<ViolationsDto> = use_signal(|| Default::default());
    let errors = use_signal(|| vec![]);
    let disabled = use_signal(|| false);

    rsx! {
        form {
            action:"#",
            class:"space-y-6",
            "novalidate": true,
            onsubmit: move |_| {
                let processor = processor.clone();
                let values = values.clone();
                let violations = violations.clone();
                let errors = errors.clone();
                let disabled = disabled.clone();
                let auth_token = auth_token.clone();

                spawn(async move {
                    processor.submit(
                        auth_token,
                        values,

                        violations,
                        errors,
                        disabled
                    ).await;
                });
            },

            ErrorBlock {errors: errors()}

            ChangePasswordSchemaInput {
                change_password_schema: values().into(),
                violations: violations(),
                oninput: move |event: ChangePasswordSchema| {
                    values.set(event);
                },
            }

            Submit {
                title: "Submit".to_string(),
                disabled: disabled(),
            }
        }
    }
}
