use crate::dto::response::{
    Status400Response, Status409Response, UserRegisteredSuccessfullyResponse,
};
use crate::dto::schema::RegistrationWithPasswordSchema;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use crate::dto::schema::UserName;

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct UserRegistersWithPasswordFormData {
    pub values: Signal<RegistrationWithPasswordSchema>,
    pub violations: Signal<ViolationsDto>,
    pub errors: Signal<Vec<String>>,
    pub disabled: Signal<bool>,
}

impl UserRegistersWithPasswordFormData {
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
pub struct UserRegistersWithPasswordFormProps {
    pub on_submit: EventHandler<FormEvent>,
    pub form_data: UserRegistersWithPasswordFormData,
}

pub struct UserRegistersWithPasswordCallbacks {
    pub on_error: EventHandler<String>,

    pub on_user_registered_successfully: EventHandler<UserRegisteredSuccessfullyResponse>,
    pub on_status_400: EventHandler<Status400Response>,
    pub on_status_409: EventHandler<Status409Response>,
}

impl Default for UserRegistersWithPasswordCallbacks {
    fn default() -> Self {
        Self {
            on_error: EventHandler::new(move |e: String| {
                dioxus_logger::tracing::error!("API error: `{:?}`", e);
            }),

            on_user_registered_successfully: EventHandler::new(
                move |_response: UserRegisteredSuccessfullyResponse| {},
            ),
            on_status_400: EventHandler::new(move |_response: Status400Response| {}),
            on_status_409: EventHandler::new(move |_response: Status409Response| {}),
        }
    }
}
