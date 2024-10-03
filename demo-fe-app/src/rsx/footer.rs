use dioxus::prelude::*;

#[component]
pub(crate) fn Footer() -> Element {
    rsx! {
        footer {
            class:"relative z-10 pt-8 bg-gray-100 dark:bg-gray-dark md:pt-10 lg:pt-12",
            div { class:"container",
                div { class:"-mx-4 flex flex-wrap",
                    div { class:"w-full px-4 md:w-1/2 lg:w-4/12 xl:w-5/12",
                        div { class:"mb-12 max-w-360 lg:mb-16",
                            div { class:"mb-12 lg:mb-16",
                                div { class:"mb-5 text-xl font-bold text-black dark:text-white",
                                    "Follow Us"
                                }
                                ul {
                                    li {
                                        a { class: "text-primary ", href:"#", "LinkedIn" },
                                    }
                                    li {
                                        a { class: "text-primary ", href:"#", "GitLab" },
                                    }
                                    li {
                                        a { class: "text-primary ", href:"#", "GitHub" },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
