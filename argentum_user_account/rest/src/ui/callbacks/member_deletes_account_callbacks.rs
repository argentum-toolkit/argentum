use crate::dto::response::Status204Response;
use crate::dto::response::Status400Response;
use crate::dto::response::Status401Response;

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MemberDeletesAccountCallbacks {
    pub on_error: EventHandler<String>,

    pub on_status_204: EventHandler<Status204Response>,
    pub on_status_400: EventHandler<Status400Response>,
    pub on_status_401: EventHandler<Status401Response>,
}

impl Default for MemberDeletesAccountCallbacks {
    fn default() -> Self {
        Self {
            on_error: EventHandler::new(move |e: String| {
                dioxus_logger::tracing::error!("API error: `{e:?}`");
            }),

            on_status_204: EventHandler::new(move |_response: Status204Response| {}),
            on_status_400: EventHandler::new(move |_response: Status400Response| {}),
            on_status_401: EventHandler::new(move |_response: Status401Response| {}),
        }
    }
}
