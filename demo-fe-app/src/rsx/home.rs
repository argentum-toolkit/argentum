use crate::route::Route;
use crate::server::{get_server_data, post_server_data};
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        section {
            div { class:"container",
                div {class:"-mx-4 flex flex-wrap justify-center",
                    div {class:"w-full px-4 lg:w-8/12",
                        div {
                            h2 { class:"mb-8 text-3xl font-bold leading-tight text-black dark:text-white sm:text-4xl sm:leading-tight",
                                "Argentum ToolKit demo fullstack application"
                            }
                            div {
                                p { class: "mb-10 text-base font-medium leading-relaxed sm:text-lg sm:leading-relaxed lg:text-base lg:leading-relaxed xl:text-lg xl:leading-relaxed",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean et elit at arcu euismod pellentesque eget sed urna. Duis malesuada id tortor quis interdum. In enim sapien, blandit in efficitur ut, rutrum vel arcu. Suspendisse id molestie mauris, sit amet mollis est. Aenean fringilla eros eget commodo pharetra. Nam pretium neque metus, ut rutrum nisi accumsan vitae. Maecenas vulputate arcu justo, in bibendum elit aliquam sed. Sed ante magna, vulputate auctor augue at, luctus malesuada ante. Duis congue elit et ultricies consectetur. Integer vel dui erat. Praesent nec urna lorem. Pellentesque ac ex libero. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos."
                                }
                                p { class: "mb-10 text-base font-medium leading-relaxed sm:text-lg sm:leading-relaxed lg:text-base lg:leading-relaxed xl:text-lg xl:leading-relaxed",
                                    "Nullam consequat consequat elit at lobortis. Nullam ut augue faucibus nisl auctor porttitor eu id urna. Nulla at iaculis libero. Ut semper diam sed rutrum volutpat. Nam tristique turpis ac tristique dapibus. Vestibulum vitae tincidunt arcu. Mauris sodales lectus et magna faucibus pretium. Nunc hendrerit venenatis tellus eget ultrices. Nunc pulvinar eu erat quis lobortis. Integer massa neque, porttitor eu sapien sit amet, elementum accumsan metus. Cras luctus ut urna convallis malesuada."
                                }
                                p { class: "mb-10 text-base font-medium leading-relaxed sm:text-lg sm:leading-relaxed lg:text-base lg:leading-relaxed xl:text-lg xl:leading-relaxed",
                                    "Donec hendrerit quam sed augue pellentesque, vel malesuada erat euismod. Pellentesque cursus ornare semper. Duis porttitor vehicula purus quis aliquam. Vivamus vehicula quis lacus vel auctor. Etiam elementum dui justo, et maximus massa imperdiet vel. Praesent in varius diam, sit amet auctor dui. Mauris a justo non sapien varius placerat ut quis ex. Ut est nisl, hendrerit sit amet rutrum at, dictum id est."
                                }
                                p { class: "mb-10 text-base font-medium leading-relaxed sm:text-lg sm:leading-relaxed lg:text-base lg:leading-relaxed xl:text-lg xl:leading-relaxed",
                                    "Aliquam odio mi, feugiat at egestas vitae, vehicula eget ligula. Quisque vehicula accumsan massa in mollis. Praesent et feugiat dui, a dapibus libero. Donec auctor aliquam porttitor. Etiam et nisl ac ipsum aliquam consectetur ut ac quam. Nulla facilisi. Fusce libero magna, malesuada et dui sit amet, sodales blandit felis. Vestibulum eget arcu facilisis, varius dui a, venenatis mi."
                                }
                            }
                        }
                    }
                }
            }
        }
        // div {
        //     button {
        //         onclick: move |_| async move {
        //             if let Ok(data) = get_server_data().await {
        //                 tracing::info!("Client received: {}", data);
        //                 text.set(data.clone());
        //                 post_server_data(data).await.unwrap();
        //             }
        //         },
        //         "Get Server Data"
        //     }
        //     p { "Server data: {text}"}
        // }
    }
}
