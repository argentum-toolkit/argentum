use dioxus::prelude::*;

#[component]
pub fn ErrorBlock(errors: Vec<String>) -> Element {
    rsx! {
        for e in errors {
            div {
                class: "mt-4 text-sm text-red-700 dark:text-red-500",
                "{e}",
            }
        }
    }
}
