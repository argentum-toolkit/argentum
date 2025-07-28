use crate::dto::response::Status400Response;
use crate::dto::response::Status409Response;
use crate::dto::response::UserRegisteredSuccessfullyResponse;

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
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
