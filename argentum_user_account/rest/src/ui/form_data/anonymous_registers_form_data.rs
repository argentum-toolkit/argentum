use crate::dto::response::AnonymousRegisteredSuccessfullyResponse;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AnonymousRegistersFormData {
    pub violations: Signal<ViolationsDto>,
    pub errors: Signal<Vec<String>>,
    pub disabled: Signal<bool>,
}

impl AnonymousRegistersFormData {
    pub fn new() -> Self {
        Self {
            violations: use_signal(|| Default::default()),
            errors: use_signal(|| vec![]),
            disabled: use_signal(|| false),
        }
    }
}

#[deprecated(since = "0.3.0", note = "please use `*Callbacks` instead")]
#[derive(Clone, PartialEq, Props)]
pub struct AnonymousRegistersFormProps {
    pub on_submit: EventHandler<FormEvent>,
    pub form_data: AnonymousRegistersFormData,
}

pub struct AnonymousRegistersCallbacks {
    pub on_error: EventHandler<String>,

    pub on_anonymous_registered_successfully: EventHandler<AnonymousRegisteredSuccessfullyResponse>,
}

impl Default for AnonymousRegistersCallbacks {
    fn default() -> Self {
        Self {
            on_error: EventHandler::new(move |e: String| {
                dioxus_logger::tracing::error!("API error: `{:?}`", e);
            }),

            on_anonymous_registered_successfully: EventHandler::new(
                move |_response: AnonymousRegisteredSuccessfullyResponse| {},
            ),
        }
    }
}
