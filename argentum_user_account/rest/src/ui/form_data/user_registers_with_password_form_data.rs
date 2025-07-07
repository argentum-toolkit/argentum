use crate::dto::response::{
    Status400Response, Status409Response, UserRegisteredSuccessfullyResponse,
};
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use crate::dto::schema::UserName;

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct Values {
    pub email: Signal<String>,
    pub name: Signal<UserName>,
    pub password: Signal<String>,
    pub terms: Signal<bool>,
}

impl Values {
    pub fn new() -> Self {
        Self {
            email: use_signal(|| String::default()),
            name: use_signal(|| UserName::default()),
            password: use_signal(|| String::default()),
            terms: use_signal(|| bool::default()),
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct RsxViolations {
    pub email: Signal<Option<ViolationsDto>>,
    pub name: Signal<Option<ViolationsDto>>,
    pub password: Signal<Option<ViolationsDto>>,
    pub terms: Signal<Option<ViolationsDto>>,
}

impl RsxViolations {
    pub fn new() -> Self {
        Self {
            email: use_signal(|| None),
            name: use_signal(|| None),
            password: use_signal(|| None),
            terms: use_signal(|| None),
        }
    }

    pub fn clear(&mut self) {
        self.email.set(None);
        self.name.set(None);
        self.password.set(None);
        self.terms.set(None);
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct UserRegistersWithPasswordFormData {
    pub values: Values,
    pub violations: RsxViolations,
    pub errors: Signal<Vec<String>>,
    pub disabled: Signal<bool>,
}

impl UserRegistersWithPasswordFormData {
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
pub struct UserRegistersWithPasswordProps {
    #[props(default = "".to_string())]
    pub id_prefix: String,
    #[props(default = EventHandler::new(move |e: String| {dioxus_logger::tracing::error!("API error: `{:?}`", e);}))]
    pub on_error: EventHandler<String>,

    #[props(default = EventHandler::new(move |_response: UserRegisteredSuccessfullyResponse| {}))]
    pub on_user_registered_successfully: EventHandler<UserRegisteredSuccessfullyResponse>,
    #[props(default = EventHandler::new(move |_response: Status400Response| {}))]
    pub on_status_400: EventHandler<Status400Response>,
    #[props(default = EventHandler::new(move |_response: Status409Response| {}))]
    pub on_status_409: EventHandler<Status409Response>,
}

#[derive(Clone, PartialEq, Props)]
pub struct UserRegistersWithPasswordFormProps {
    pub on_submit: EventHandler<FormEvent>,
    pub form_data: UserRegistersWithPasswordFormData,
}
