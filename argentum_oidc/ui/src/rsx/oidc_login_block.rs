use argentum_standard_ui::rsx::component::ErrorBlock;
use dioxus::prelude::*;

use crate::rsx::{github::GitHubButton, google::GoogleButton};

#[component]
pub fn OidcLoginBlock() -> Element {
    let errors = use_signal(Vec::new);

    rsx! {
        div {
            class: "flex flex-col items-center justify-center gap-4",
            div {
                class: "divider",
                "OR"
            }

            ErrorBlock {errors: errors()}

            GoogleButton {errors: errors}
            GitHubButton { errors: errors}
        }
    }
}
