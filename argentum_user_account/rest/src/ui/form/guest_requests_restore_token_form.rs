use crate::dto::schema::RequestRestoreTokenSchema;
use crate::ui::form_processor::GuestRequestsRestoreTokenFormProcessor;
use crate::ui::input::RequestRestoreTokenSchemaInput;

use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use argentum_standard_ui::rsx::component::ErrorBlock;
use argentum_standard_ui::rsx::form::Submit;

use dioxus::prelude::*;

#[component]
pub fn GuestRequestsRestoreTokenForm(
    processor: GuestRequestsRestoreTokenFormProcessor,
    auth_token: String,
    request_restore_token_schema: Option<RequestRestoreTokenSchema>,
) -> Element {
    let mut values: Signal<RequestRestoreTokenSchema> = match request_restore_token_schema {
        Some(v) => use_signal(|| v),
        None => use_signal(Default::default),
    };
    let violations: Signal<ViolationsDto> = use_signal(Default::default);
    let errors = use_signal(Vec::new);
    let disabled = use_signal(|| false);

    rsx! {
        form {
            action:"#",
            class:"space-y-6",
            "novalidate": true,
            onsubmit: move |e| {
                e.prevent_default();

                let processor = processor.clone();
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

            RequestRestoreTokenSchemaInput {
                value: values(),
                violations: violations(),
                ui_wrapper: "none".to_string(),
                oninput: move |event: RequestRestoreTokenSchema| {
                    values.set(event);
                },
                processor: processor.input_processor.clone(),
            }

            div {
                class: "w-full",
                Submit {
                    title: "Request restore password link".to_string(),
                    disabled: disabled(),
                }
            }
        }
    }
}
