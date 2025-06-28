use crate::dto::response::{
    GetUserOkResponse, Status401Response, Status403Response, Status404Response,
};
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

pub struct GetUserFormData {
    pub values: Values,
    pub violations: RsxViolations,
    pub errors: Signal<Vec<String>>,
    pub disabled: Signal<bool>,
}

impl GetUserFormData {
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
pub struct GetUserProps {
    #[props(default = "".to_string())]
    pub id_prefix: String,
    #[props(default = EventHandler::new(move |e: String| {dioxus_logger::tracing::error!("API error: `{:?}`", e);}))]
    pub on_error: EventHandler<String>,

    #[props(default = EventHandler::new(move |_response: GetUserOkResponse| {}))]
    pub on_get_user_ok: EventHandler<GetUserOkResponse>,
    #[props(default = EventHandler::new(move |_response: Status401Response| {}))]
    pub on_status_401: EventHandler<Status401Response>,
    #[props(default = EventHandler::new(move |_response: Status403Response| {}))]
    pub on_status_403: EventHandler<Status403Response>,
    #[props(default = EventHandler::new(move |_response: Status404Response| {}))]
    pub on_status_404: EventHandler<Status404Response>,
}
