use dioxus::prelude::*;

#[component]
pub fn Contacts() -> Element {
    rsx! {
        div {
            h1 { "Contact Argentum Skeleton" }
            p { "Get in touch with the Argentum Skeleton team." }
            p { "Email: contact@argentum-toolkit.com" }
            p { "GitHub: https://github.com/argentum-toolkit/argentum (primary repo)" }
            p { "GitLab: https://gitlab.com/argentum-toolkit/argentum" }
        }
    }
}
