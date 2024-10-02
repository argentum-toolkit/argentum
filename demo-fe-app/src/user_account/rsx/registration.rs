use crate::user_account::rsx::user_name::UserName;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn Registration() -> Element {
    rsx! {
        form {
            onsubmit: move |event| {info!("Submitted! {event:?} ")},
            div {
                "Email:"
                input { name: "email"}
            }
            UserName {}
            div {
                "Email:"
                input { name: "email"}
            }


        }
    }
}
