use crate::dto::schema::DeleteAccountSchema;
use crate::ui::form_processor::MemberDeletesAccountFormProcessor;
use crate::ui::input::DeleteAccountSchemaInput;

use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use argentum_standard_ui::rsx::component::ErrorBlock;
use argentum_standard_ui::rsx::form::Submit;

use dioxus::prelude::*;

#[component]
pub fn MemberDeletesAccountForm(
    processor: MemberDeletesAccountFormProcessor,
    auth_token: String,
    delete_account_schema: Option<DeleteAccountSchema>,
) -> Element {
    let mut values: Signal<DeleteAccountSchema> = match delete_account_schema {
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

            DeleteAccountSchemaInput {
                value: values(),
                violations: violations(),
                ui_wrapper: "".to_string(),
                oninput: move |event: DeleteAccountSchema| {
                    values.set(event);
                },
                processor: processor.input_processor.clone(),
            }

            div {
                class: "w-full",
                Submit {
                    title: "Delete account".to_string(),
                    disabled: disabled(),
                }
            }
        }
    }
}
