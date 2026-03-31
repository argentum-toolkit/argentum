use crate::dto::schema::LoginWithPasswordSchema;
use crate::ui::form_processor::MemberLoginsWithPasswordFormProcessor;
use crate::ui::input::LoginWithPasswordSchemaInput;

use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use argentum_standard_ui::rsx::component::ErrorBlock;
use argentum_standard_ui::rsx::form::Submit;

use dioxus::prelude::*;

#[component]
pub fn MemberLoginsWithPasswordForm(
    processor: MemberLoginsWithPasswordFormProcessor,
    auth_token: String,
    login_with_password_schema: Option<LoginWithPasswordSchema>,
) -> Element {
    let mut values: Signal<LoginWithPasswordSchema> = match login_with_password_schema {
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

            LoginWithPasswordSchemaInput {
                value: values(),
                violations: violations(),
                oninput: move |event: LoginWithPasswordSchema| {
                    values.set(event);
                },
                processor: processor.input_processor.clone(),
            }

            Submit {
                title: "Sign In".to_string(),
                disabled: disabled(),
            }
        }
    }
}
