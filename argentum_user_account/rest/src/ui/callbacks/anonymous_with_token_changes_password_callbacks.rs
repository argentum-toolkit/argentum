use crate::dto::response::EmptyOkResponse;
use crate::dto::response::Status400Response;
use crate::dto::response::Status401Response;

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AnonymousWithTokenChangesPasswordCallbacks {
    pub on_error: EventHandler<String>,

    pub on_empty_ok: EventHandler<EmptyOkResponse>,
    pub on_status_400: EventHandler<Status400Response>,
    pub on_status_401: EventHandler<Status401Response>,
}

impl Default for AnonymousWithTokenChangesPasswordCallbacks {
    fn default() -> Self {
        Self {
            on_error: EventHandler::new(move |e: String| {
                dioxus_logger::tracing::error!("API error: `{e:?}`");
            }),

            on_empty_ok: EventHandler::new(move |_response: EmptyOkResponse| {}),
            on_status_400: EventHandler::new(move |_response: Status400Response| {}),
            on_status_401: EventHandler::new(move |_response: Status401Response| {}),
        }
    }
}
