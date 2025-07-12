use crate::dto::response::{
    GetUserOkResponse, Status401Response, Status403Response, Status404Response,
};
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct GetUserFormData {
    pub violations: Signal<ViolationsDto>,
    pub errors: Signal<Vec<String>>,
    pub disabled: Signal<bool>,
}

impl GetUserFormData {
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
pub struct GetUserFormProps {
    pub on_submit: EventHandler<FormEvent>,
    pub form_data: GetUserFormData,
}

pub struct GetUserCallbacks {
    pub on_error: EventHandler<String>,

    pub on_get_user_ok: EventHandler<GetUserOkResponse>,
    pub on_status_401: EventHandler<Status401Response>,
    pub on_status_403: EventHandler<Status403Response>,
    pub on_status_404: EventHandler<Status404Response>,
}

impl Default for GetUserCallbacks {
    fn default() -> Self {
        Self {
            on_error: EventHandler::new(move |e: String| {
                dioxus_logger::tracing::error!("API error: `{:?}`", e);
            }),

            on_get_user_ok: EventHandler::new(move |_response: GetUserOkResponse| {}),
            on_status_401: EventHandler::new(move |_response: Status401Response| {}),
            on_status_403: EventHandler::new(move |_response: Status403Response| {}),
            on_status_404: EventHandler::new(move |_response: Status404Response| {}),
        }
    }
}
