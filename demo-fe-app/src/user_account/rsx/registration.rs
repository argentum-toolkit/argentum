use crate::route::Route;
use crate::user_account::rsx::user_name::UserName;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn Registration() -> Element {
    rsx! {
        section {
            div { class: "container",
                div { class:"pb-40",
                    h2 { class:"mb-3 text-center text-2xl font-bold text-black dark:text-white sm:text-3xl", "Create your account"}

                    div { class:"mt-10 sm:mx-auto sm:w-full sm:max-w-sm",
                        form { class:"space-y-6", action:"#", method:"POST",
                            onsubmit: move |event| {info!("Submitted! {event:?} ")},
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

                            UserName {}

                            div { class: "mb-8 flex",
                                label {
                                    "htmlFor": "checkboxLabel",
                                    class: "flex cursor-pointer select-none text-sm font-medium text-body-color",
                                    div { class: "relative",
                                        input { "type":"checkbox", id: "checkboxLabel", class: "sr-only"}
                                        div { class: "box mr-4 mt-1 flex h-5 w-5 items-center justify-center rounded border border-body-color border-opacity-20 dark:border-white dark:border-opacity-10",
                                            span {
                                                class: "opacity-0",
                                                svg {
                                                    width:"11",
                                                    height:"8",
                                                    "viewBox":"0 0 11 8",
                                                    fill:"none",
                                                    xmlns:"http://www.w3.org/2000/svg",
                                                    path {
                                                        d:"M10.0915 0.951972L10.0867 0.946075L10.0813 0.940568C9.90076 0.753564 9.61034 0.753146 9.42927 0.939309L4.16201 6.22962L1.58507 3.63469C1.40401 3.44841 1.11351 3.44879 0.932892 3.63584C0.755703 3.81933 0.755703 4.10875 0.932892 4.29224L0.932878 4.29225L0.934851 4.29424L3.58046 6.95832C3.73676 7.11955 3.94983 7.2 4.1473 7.2C4.36196 7.2 4.55963 7.11773 4.71406 6.9584L10.0468 1.60234C10.2436 1.4199 10.2421 1.1339 10.0915 0.951972ZM4.2327 6.30081L4.2317 6.2998C4.23206 6.30015 4.23237 6.30049 4.23269 6.30082L4.2327 6.30081Z",
                                                        fill:"#3056D3",
                                                        stroke:"#3056D3",
                                                        "strokeWidth":"0.4",
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    span {
                                        "By creating account means you agree to the"
                                        a {href:"#0", class:"text-primary hover:underline",
                                            "Terms and Conditions"
                                        }
                                        ", and our"
                                        a {href:"#0", class:"text-primary hover:underline",
                                            "Privacy Policy"
                                        }
                                    }
                                }
                            }

                            div {
                                button {
                                    "type":"submit",
                                    class:"shadow-submit dark:shadow-submit-dark flex w-full items-center justify-center rounded-sm bg-primary px-9 py-4 text-base font-medium text-white duration-300 hover:bg-primary/90",
                                    "Sign Up"
                                }
                            }
                        }

                        p { class: "text-center text-base font-medium text-body-color py-8 dark:text-body-color-dark",
                            "Already here?"
                            Link { class: "text-primary hover:underline pl-2", to: Route::Login {}, "Sign In" }
                        }
                    }
                }
            }
        }
    }
}
