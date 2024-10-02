use dioxus::prelude::*;

#[component]
pub(crate) fn Footer() -> Element {
    rsx! {
        div {
            "(C) Argentum ToolKit"
        }
        div {
            "Contacts us"
        }
        div {
            "Connect with: "
            strong {
                "in "
            }
            strong {
                "GitLab "
            }
            strong {
                "GitHub"
            }

        }
    }
}
