use dioxus::prelude::*;

#[component]
pub fn Contacts() -> Element {
    rsx! {
        div {
            class: "prose max-w-none",
            div {class: "card p-6",
                h1 { "Contact Argentum ToolKit" }
                p { "Get in touch with the Argentum ToolKit team." }
                p { "Email: contact@argentum-toolkit.com" }
                p { "GitHub: https://github.com/argentum-toolkit/argentum (primary repo)" }
                p { "GitLab: https://gitlab.com/argentum-toolkit/argentum" }
            }
        }
    }
}
