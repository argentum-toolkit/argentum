use crate::dto::response::{EmptyOkResponse, Status400Response, Status401Response};
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct Values {
    pub password: Signal<String>,
    pub token: Signal<String>,
}

impl Values {
    pub fn new() -> Self {
        Self {
            password: use_signal(|| String::default()),
            token: use_signal(|| String::default()),
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct AnonymousWithTokenChangesPasswordFormData {
    pub values: Values,
    pub violations: ViolationsDto,
    pub errors: Signal<Vec<String>>,
    pub disabled: Signal<bool>,
}

impl AnonymousWithTokenChangesPasswordFormData {
    pub fn new() -> Self {
        Self {
            values: Values::new(),
            violations: ViolationsDto::new(vec![], None),
            errors: use_signal(|| vec![]),
            disabled: use_signal(|| false),
        }
    }
}

#[deprecated(since = "0.3.0", note = "please use `*Callbacks` instead")]
#[derive(Clone, PartialEq, Props)]
pub struct AnonymousWithTokenChangesPasswordFormProps {
    pub on_submit: EventHandler<FormEvent>,
    pub form_data: AnonymousWithTokenChangesPasswordFormData,
}

pub struct AnonymousWithTokenChangesPasswordCallbacks {
    pub on_error: EventHandler<String>,

    pub on_empty_ok: EventHandler<EmptyOkResponse>,
    pub on_status_400: EventHandler<Status400Response>,
    pub on_status_401: EventHandler<Status401Response>,
}

impl Default for AnonymousWithTokenChangesPasswordCallbacks {
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
