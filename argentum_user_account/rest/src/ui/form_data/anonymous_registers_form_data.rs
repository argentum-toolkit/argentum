use crate::dto::response::AnonymousRegisteredSuccessfullyResponse;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use dioxus::prelude::*;

#[derive(Clone)]
pub struct Values {}

impl Values {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone)]
pub struct RsxViolations {}

impl RsxViolations {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct AnonymousRegistersFormData {
    pub values: Values,
    pub violations: RsxViolations,
    pub errors: Signal<Vec<String>>,
    pub disabled: Signal<bool>,
}

impl AnonymousRegistersFormData {
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
pub struct LoginWithPasswordProps {
    #[props(default = "".to_string())]
    pub id_prefix: String,
    #[props(default = EventHandler::new(move |e: String| {dioxus_logger::tracing::error!("API error: `{:?}`", e);}))]
    pub on_error: EventHandler<String>,

    #[props(default = EventHandler::new(move |_response: AnonymousRegisteredSuccessfullyResponse| {}))]
    pub on_anonymous_registered_successfully: EventHandler<AnonymousRegisteredSuccessfullyResponse>,
}
