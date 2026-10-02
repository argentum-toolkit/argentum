use dioxus::prelude::*;

#[component]
pub fn Footer(title: String, links: Vec<(String, String)>) -> Element {
    rsx! {
        footer {
            class:"footer p-4 bg-base-200 text-center text-base-content",
            nav {
                div { class: "footer-title text-base-content", "{title}"}
                for (label, link) in links.iter() {
                     a { class: "link link-hover ", href:"{link}", "{label}" },
                }
            }
        }
    }
}
