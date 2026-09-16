use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            h1 { "About Argentum Skeleton" }
            p { "Argentum is a modular Rust toolkit for building applications." }
            p { "This skeleton UI layer uses Dioxus with RSX components." }
            p { "It integrates db, rest, user, event and encryption modules." }
        }
    }
}
