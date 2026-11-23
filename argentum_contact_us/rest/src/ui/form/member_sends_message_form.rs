use crate::dto::schema::MemberSendsMessageSchema;
use crate::ui::form_processor::MemberSendsMessageFormProcessor;
use crate::ui::input::MemberSendsMessageSchemaInput;

use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use argentum_standard_ui::rsx::component::ErrorBlock;
use argentum_standard_ui::rsx::form::Submit;

use dioxus::prelude::*;

#[component]
pub fn MemberSendsMessageForm(
    processor: MemberSendsMessageFormProcessor,
    auth_token: String,
    member_sends_message_schema: Option<MemberSendsMessageSchema>,
) -> Element {
    let mut values: Signal<MemberSendsMessageSchema> = match member_sends_message_schema {
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

            MemberSendsMessageSchemaInput {
                value: values(),
                violations: violations(),
                ui_wrapper: "".to_string(),
                oninput: move |event: MemberSendsMessageSchema| {
                    values.set(event);
                },
                processor: processor.input_processor.clone(),
            }

            div {
                class: "w-full",
                Submit {
                    title: "Send".to_string(),
                    disabled: disabled(),
                }
            }
        }
    }
}
