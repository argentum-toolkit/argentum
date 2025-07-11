use crate::dto::response::{
    Status400Response, Status401Response, UserLoggedInSuccessfullyResponse,
};
use crate::dto::schema::LoginWithPasswordSchema;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct UserLoginsWithPasswordFormData {
    pub values: Signal<LoginWithPasswordSchema>,
    pub violations: Signal<ViolationsDto>,
    pub errors: Signal<Vec<String>>,
    pub disabled: Signal<bool>,
}

impl UserLoginsWithPasswordFormData {
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
pub struct UserLoginsWithPasswordFormProps {
    pub on_submit: EventHandler<FormEvent>,
    pub form_data: UserLoginsWithPasswordFormData,
}

pub struct UserLoginsWithPasswordCallbacks {
    pub on_error: EventHandler<String>,

    pub on_user_logged_in_successfully: EventHandler<UserLoggedInSuccessfullyResponse>,
    pub on_status_400: EventHandler<Status400Response>,
    pub on_status_401: EventHandler<Status401Response>,
}

impl Default for UserLoginsWithPasswordCallbacks {
    fn default() -> Self {
        Self {
            on_error: EventHandler::new(move |e: String| {
                dioxus_logger::tracing::error!("API error: `{:?}`", e);
            }),

            on_user_logged_in_successfully: EventHandler::new(
                move |_response: UserLoggedInSuccessfullyResponse| {},
            ),
            on_status_400: EventHandler::new(move |_response: Status400Response| {}),
            on_status_401: EventHandler::new(move |_response: Status401Response| {}),
        }
    }
}
