use crate::rsx::component::Spinner;
use dioxus::prelude::*;

#[component]
pub fn Submit(title: String, disabled: bool) -> Element {
    rsx! {
        button {
            "type":"submit",
            class:"shadow-submit dark:shadow-submit-dark flex w-full items-center justify-center rounded-sm bg-primary px-9 py-4 text-base font-medium text-white duration-300 hover:bg-primary/90 disabled:opacity-25",
            disabled: "{disabled}",

            if disabled {
                Spinner {size: 20}
            }

            "{title}",
        }
    }
}
