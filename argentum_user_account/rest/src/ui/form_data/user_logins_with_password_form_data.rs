use crate::dto::response::{
    Status400Response, Status401Response, UserLoggedInSuccessfullyResponse,
};
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use dioxus::prelude::*;

#[derive(Clone)]
pub struct Values {
    pub email: Signal<String>,
    pub password: Signal<String>,
}

impl Values {
    pub fn new() -> Self {
        Self {
            email: use_signal(|| String::default()),
            password: use_signal(|| String::default()),
        }
    }
}

#[derive(Clone)]
pub struct RsxViolations {
    pub email: Signal<Option<ViolationsDto>>,
    pub password: Signal<Option<ViolationsDto>>,
}

impl RsxViolations {
    pub fn new() -> Self {
        Self {
            email: use_signal(|| None),
            password: use_signal(|| None),
        }
    }
}

pub struct UserLoginsWithPasswordFormData {
    pub values: Values,
    pub violations: RsxViolations,
    pub errors: Signal<Vec<String>>,
    pub disabled: Signal<bool>,
}

impl UserLoginsWithPasswordFormData {
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
pub struct UserLoginsWithPasswordProps {
    #[props(default = "".to_string())]
    pub id_prefix: String,
    #[props(default = EventHandler::new(move |e: String| {dioxus_logger::tracing::error!("API error: `{:?}`", e);}))]
    pub on_error: EventHandler<String>,

    #[props(default = EventHandler::new(move |_response: UserLoggedInSuccessfullyResponse| {}))]
    pub on_user_logged_in_successfully: EventHandler<UserLoggedInSuccessfullyResponse>,
    #[props(default = EventHandler::new(move |_response: Status400Response| {}))]
    pub on_status_400: EventHandler<Status400Response>,
    #[props(default = EventHandler::new(move |_response: Status401Response| {}))]
    pub on_status_401: EventHandler<Status401Response>,
}
