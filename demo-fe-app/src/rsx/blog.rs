use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        section {
            div { class:"container",
                div {class:"-mx-4 justify-center",
                    div {class:"w-full px-4 lg:w-8/12",
                        div {
                            h2 { class:"mb-8 text-3xl font-bold leading-tight text-black dark:text-white sm:text-4xl sm:leading-tight",
                                "Blog"
                            }
                            div {
                                "Blog post {id}"
                            }
                        }
                    }
                }
            }
        }
    }
}
