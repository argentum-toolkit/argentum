use crate::dto::response::AnonymousRegisteredSuccessfullyResponse;

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AnonymousRegistersCallbacks {
    pub on_error: EventHandler<String>,

    pub on_anonymous_registered_successfully: EventHandler<AnonymousRegisteredSuccessfullyResponse>,
}

impl Default for AnonymousRegistersCallbacks {
    fn default() -> Self {
        Self {
            on_error: EventHandler::new(move |e: String| {
                dioxus_logger::tracing::error!("API error: `{e:?}`");
            }),

            on_anonymous_registered_successfully: EventHandler::new(
                move |_response: AnonymousRegisteredSuccessfullyResponse| {},
            ),
        }
    }
}
