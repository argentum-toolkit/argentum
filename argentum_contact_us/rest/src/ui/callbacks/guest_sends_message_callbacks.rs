use crate::dto::response::MessageCreatedSuccessfullyResponse;
use crate::dto::response::Status400Response;

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct GuestSendsMessageCallbacks {
    pub on_error: EventHandler<String>,

    pub on_message_created_successfully: EventHandler<MessageCreatedSuccessfullyResponse>,
    pub on_status_400: EventHandler<Status400Response>,
}

impl Default for GuestSendsMessageCallbacks {
    fn default() -> Self {
        Self {
            on_error: EventHandler::new(move |e: String| {
                dioxus_logger::tracing::error!("API error: `{e:?}`");
            }),

            on_message_created_successfully: EventHandler::new(
                move |_response: MessageCreatedSuccessfullyResponse| {},
            ),
            on_status_400: EventHandler::new(move |_response: Status400Response| {}),
        }
    }
}
