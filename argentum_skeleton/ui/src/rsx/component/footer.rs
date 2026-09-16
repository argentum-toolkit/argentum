use dioxus::prelude::*;

#[component]
pub fn Footer(title: String, links: Vec<(String, String)>) -> Element {
    rsx! {
        footer {
            class:"footer p-4 bg-base-300 text-center",
            nav {
                h6 { class: "footer-title", "{title}"}
                for (label, link) in links.iter() {
                     a { class: "link link-hover ", href:"{link}", "{label}" },
                }
            }
        }
    }
}
