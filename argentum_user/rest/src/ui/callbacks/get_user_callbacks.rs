use crate::dto::response::GetUserOkResponse;
use crate::dto::response::Status401Response;
use crate::dto::response::Status403Response;
use crate::dto::response::Status404Response;

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
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
