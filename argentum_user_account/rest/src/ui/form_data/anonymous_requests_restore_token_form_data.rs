use crate::dto::response::{EmptyOkResponse, Status400Response, Status401Response};
use crate::dto::schema::RequestRestoreTokenSchema;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AnonymousRequestsRestoreTokenFormData {
    pub values: Signal<RequestRestoreTokenSchema>,
    pub violations: Signal<ViolationsDto>,
    pub errors: Signal<Vec<String>>,
    pub disabled: Signal<bool>,
}

impl AnonymousRequestsRestoreTokenFormData {
    pub fn new() -> Self {
        Self {
            values: use_signal(|| Default::default()),
            violations: use_signal(|| Default::default()),
            errors: use_signal(|| vec![]),
            disabled: use_signal(|| false),
        }
    }
}

#[deprecated(since = "0.3.0", note = "please use `*Callbacks` instead")]
#[derive(Clone, PartialEq, Props)]
pub struct AnonymousRequestsRestoreTokenFormProps {
    pub on_submit: EventHandler<FormEvent>,
    pub form_data: AnonymousRequestsRestoreTokenFormData,
}

pub struct AnonymousRequestsRestoreTokenCallbacks {
    pub on_error: EventHandler<String>,

    pub on_empty_ok: EventHandler<EmptyOkResponse>,
    pub on_status_400: EventHandler<Status400Response>,
    pub on_status_401: EventHandler<Status401Response>,
}

impl Default for AnonymousRequestsRestoreTokenCallbacks {
    fn default() -> Self {
        Self {
            on_error: EventHandler::new(move |e: String| {
                dioxus_logger::tracing::error!("API error: `{:?}`", e);
            }),

            on_empty_ok: EventHandler::new(move |_response: EmptyOkResponse| {}),
            on_status_400: EventHandler::new(move |_response: Status400Response| {}),
            on_status_401: EventHandler::new(move |_response: Status401Response| {}),
        }
    }
}
