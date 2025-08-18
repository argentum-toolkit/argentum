use crate::dto::response::Status400Response;
use crate::dto::response::Status401Response;
use crate::dto::response::UserLoggedInSuccessfullyResponse;

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
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
                dioxus_logger::tracing::error!("API error: `{e:?}`");
            }),

            on_user_logged_in_successfully: EventHandler::new(
                move |_response: UserLoggedInSuccessfullyResponse| {},
            ),
            on_status_400: EventHandler::new(move |_response: Status400Response| {}),
            on_status_401: EventHandler::new(move |_response: Status401Response| {}),
        }
    }
}
