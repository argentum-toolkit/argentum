use crate::dto::schema::LoginWithPasswordSchema;
use crate::ui::form_processor::UserLoginsWithPasswordFormProcessor;
use crate::ui::input::LoginWithPasswordSchemaInput;

use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use argentum_standard_ui::rsx::component::ErrorBlock;
use argentum_standard_ui::rsx::form::Submit;

use dioxus::prelude::*;

#[component]
pub fn UserLoginsWithPasswordForm(
    processor: UserLoginsWithPasswordFormProcessor,
    auth_token: String,
) -> Element {
    let mut values: Signal<LoginWithPasswordSchema> = use_signal(Default::default);
    let violations: Signal<ViolationsDto> = use_signal(Default::default);
    let errors = use_signal(Vec::new);
    let disabled = use_signal(|| false);

    rsx! {
        form {
            action:"#",
            class:"space-y-6",
            "novalidate": true,
            onsubmit: move |_| {
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

            LoginWithPasswordSchemaInput {
                value: values(),
                violations: violations(),
                oninput: move |event: LoginWithPasswordSchema| {
                    values.set(event);
                },
            }

            Submit {
                title: "Sign In".to_string(),
                disabled: disabled(),
            }
        }
    }
}
