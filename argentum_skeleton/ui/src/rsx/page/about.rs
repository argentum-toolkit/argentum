use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "prose max-w-none",
            div { class: "card p-6",
                h1 { "About Argentum ToolKit" }
                p { "Argentum is a modular Rust toolkit for building applications." }
                p { "This skeleton UI layer uses Dioxus with RSX components." }
                p { "It integrates db, rest, user, event and encryption modules." }
            }
        }
    }
}
