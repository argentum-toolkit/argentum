use crate::dto::schema::GuestSendsMessageSchema;
use crate::ui::form_processor::GuestSendsMessageFormProcessor;
use crate::ui::input::GuestSendsMessageSchemaInput;

use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use argentum_standard_ui::rsx::component::ErrorBlock;
use argentum_standard_ui::rsx::form::Submit;

use dioxus::prelude::*;

#[component]
pub fn GuestSendsMessageForm(
    processor: GuestSendsMessageFormProcessor,
    guest_sends_message_schema: Option<GuestSendsMessageSchema>,
) -> Element {
    let mut values: Signal<GuestSendsMessageSchema> = match guest_sends_message_schema {
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

                spawn(async move {
                    processor.submit(


                        values,
                        violations,
                        errors,
                        disabled
                    ).await;
                });
            },

            ErrorBlock {errors: errors()}

            GuestSendsMessageSchemaInput {
                value: values(),
                violations: violations(),
                ui_wrapper: "".to_string(),
                oninput: move |event: GuestSendsMessageSchema| {
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
