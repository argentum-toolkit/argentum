use dioxus::prelude::*;

#[component]
pub(crate) fn Footer() -> Element {
    rsx! {
        footer {
            class:"footer p-4 bg-base-300 text-center",
            nav {
                h6 { class: "footer-title", "Follow Us"}
                a { class: "link link-hover ", href:"#", "LinkedIn" },
                a { class: "link link-hover ", href:"#", "GitLab" },
                a { class: "link link-hover ", href:"#", "GitHub" },
            }
        }
    }
}
