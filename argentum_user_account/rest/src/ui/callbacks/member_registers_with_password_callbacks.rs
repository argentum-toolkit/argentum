use crate::dto::response::MemberRegisteredSuccessfullyResponse;
use crate::dto::response::Status400Response;
use crate::dto::response::Status409Response;

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MemberRegistersWithPasswordCallbacks {
    pub on_error: EventHandler<String>,

    pub on_member_registered_successfully: EventHandler<MemberRegisteredSuccessfullyResponse>,
    pub on_status_400: EventHandler<Status400Response>,
    pub on_status_409: EventHandler<Status409Response>,
}

impl Default for MemberRegistersWithPasswordCallbacks {
    fn default() -> Self {
        Self {
            on_error: EventHandler::new(move |e: String| {
                dioxus_logger::tracing::error!("API error: `{e:?}`");
            }),

            on_member_registered_successfully: EventHandler::new(
                move |_response: MemberRegisteredSuccessfullyResponse| {},
            ),
            on_status_400: EventHandler::new(move |_response: Status400Response| {}),
            on_status_409: EventHandler::new(move |_response: Status409Response| {}),
        }
    }
}
