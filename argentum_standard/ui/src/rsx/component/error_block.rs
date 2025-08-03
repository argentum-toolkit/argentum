use dioxus::prelude::*;

#[component]
pub fn ErrorBlock(errors: Vec<String>) -> Element {
    rsx! {
        for e in errors {
            div {
                class: "text-error",
                "{e}",
            }
        }
    }
}
