use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        section {
            h1 { "Blog" }
            p {
                "Blog post {id}"
            }
        }
    }
}
