use crate::route::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn Login() -> Element {
    rsx! {
        section {
            div { class: "container",
                div { class:"pb-40",
                    h2 { class:"mb-3 text-center text-2xl font-bold text-black dark:text-white sm:text-3xl", "Sign in to your account"}

                    div {

                        class:"mt-10 sm:mx-auto sm:w-full sm:max-w-sm",
                        form {
                            onsubmit: move |event| {
                                let valid = event.data.valid();
                                let values = event.data.values();
                                info!("!IS_VALID! {valid:?} ");
                                info!("!Submitted! {values:?} ");
                            },
                            class:"space-y-6", action:"#", method:"POST",

                            div {
                                label { "for":"email", class:"block text-sm font-medium leading-6 text-gray-900 dark:text-body-color-dark", "Email address" }
                                div { class:"mt-2",
                                    input {
                                        id:"email", name:"email", "type":"email", autocomplete:"email", required:true,
                                        class:"border-stroke dark:text-body-color-dark dark:shadow-two w-full rounded-sm border bg-[#f8f8f8] px-6 py-3 text-base text-body-color outline-none transition-all duration-300 focus:border-primary dark:border-transparent dark:bg-[#2C303B] dark:focus:border-primary dark:focus:shadow-none"
                                    }
                                }
                            }

                            div {
                                div { class:"flex items-center justify-between",
                                    label {"for":"password", class:"block text-sm font-medium leading-6 text-gray-900 dark:text-body-color-dark", "Password"}
                                }
                                div { class:"mt-2",
                                    input {
                                        id:"password", name:"password", "type":"password", autocomplete:"current-password", required:true,
                                        class: "border-stroke dark:text-body-color-dark dark:shadow-two w-full rounded-sm border bg-[#f8f8f8] px-6 py-3 text-base text-body-color outline-none transition-all duration-300 focus:border-primary dark:border-transparent dark:bg-[#2C303B] dark:focus:border-primary dark:focus:shadow-none"
                                    }
                                }

                            }
                            div { class:"text-sm",
                                    a {href:"#", class:"text-sm font-medium text-primary hover:underline", "Forgot password?"}
                                }
                            div {
                                button {
                                    "type":"submit",
                                    class:"shadow-submit dark:shadow-submit-dark flex w-full items-center justify-center rounded-sm bg-primary px-9 py-4 text-base font-medium text-white duration-300 hover:bg-primary/90",
                                    "Sign in"
                                }
                            }
                        }
                        p { class: "text-center text-base font-medium text-body-color py-8 dark:text-body-color-dark",
                            "Don't you have an account?"
                            Link { class: "text-primary hover:underline pl-2", to: Route::Registration {}, "Sign Up" }
                        }
                    }
                }
            }
        }
    }
}
