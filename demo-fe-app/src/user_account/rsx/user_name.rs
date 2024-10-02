use dioxus::prelude::*;

#[component]
pub fn UserName() -> Element {
    let first = use_signal(|| "".to_string());
    rsx! {
        div {
            "First: {first}"
            input { name: "first"}
        }
        div {
            "Last:"
            input { name: "last"}
        }
        div {
            "Patronymic:"
            input { name: "patronymic"}
        }
    }
}
