use crate::dto::path_params::GetUserPathParams;
use crate::ui::form_processor::GetUserFormProcessor;
use std::sync::Arc;

use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use argentum_standard_ui::rsx::component::ErrorBlock;
use argentum_standard_ui::rsx::form::Submit;

use dioxus::prelude::*;

#[component]
pub fn GetUserForm(
    processor: Arc<GetUserFormProcessor>,
    auth_token: String,
    path_params: GetUserPathParams,
) -> Element {
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
                let path_params = path_params.clone();
                let violations = violations.clone();
                let errors = errors.clone();
                let disabled = disabled.clone();
                let auth_token = auth_token.clone();

                spawn(async move {
                    processor.submit(
                        auth_token,

                        path_params,
                        violations,
                        errors,
                        disabled
                    ).await;
                });
            },

            ErrorBlock {errors: errors()}


            Submit {
                title: "Submit".to_string(),
                disabled: disabled(),
            }
        }
    }
}
