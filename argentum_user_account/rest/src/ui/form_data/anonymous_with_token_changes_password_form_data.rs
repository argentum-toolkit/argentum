use crate::dto::response::{EmptyOkResponse, Status400Response, Status401Response};
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use dioxus::prelude::*;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct RsxViolations {
    pub password: Signal<Option<ViolationsDto>>,
    pub token: Signal<Option<ViolationsDto>>,
}

impl RsxViolations {
    pub fn new() -> Self {
        Self {
            password: use_signal(|| None),
            token: use_signal(|| None),
        }
    }
}

pub struct AnonymousWithTokenChangesPasswordFormData {
    pub values: Values,
    pub violations: RsxViolations,
    pub errors: Signal<Vec<String>>,
    pub disabled: Signal<bool>,
}

impl AnonymousWithTokenChangesPasswordFormData {
    pub fn new() -> Self {
        Self {
            values: Values::new(),
            violations: RsxViolations::new(),
            errors: use_signal(|| vec![]),
            disabled: use_signal(|| false),
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct AnonymousWithTokenChangesPasswordProps {
    #[props(default = "".to_string())]
    pub id_prefix: String,
    #[props(default = EventHandler::new(move |e: String| {dioxus_logger::tracing::error!("API error: `{:?}`", e);}))]
    pub on_error: EventHandler<String>,

    #[props(default = EventHandler::new(move |_response: EmptyOkResponse| {}))]
    pub on_empty_ok: EventHandler<EmptyOkResponse>,
    #[props(default = EventHandler::new(move |_response: Status400Response| {}))]
    pub on_status_400: EventHandler<Status400Response>,
    #[props(default = EventHandler::new(move |_response: Status401Response| {}))]
    pub on_status_401: EventHandler<Status401Response>,
}
