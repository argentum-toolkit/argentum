use crate::rsx::component::Spinner;
use dioxus::prelude::*;

#[component]
pub fn Submit(title: String, disabled: bool) -> Element {
    rsx! {
        button {
            "type":"submit",
            class:"btn btn-active btn-primary mt-5",
            disabled: "{disabled}",

            if disabled {
                Spinner {size: 20}
            }

            "{title}",
        }
    }
}
