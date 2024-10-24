use crate::route::Route;
use crate::rsx::dark_mode::DarkModeToggle;
#[cfg(feature = "web")]
use crate::user_account::service::ClientSideAuthenticator;
use dioxus::prelude::*;
use std::cell::RefCell;

#[component]
pub(crate) fn NavBar() -> Element {
    let mut is_user_authenticated = use_signal(|| false);

    #[cfg(feature = "web")]
    use_effect(move || {
        let authenticator = use_context::<Signal<RefCell<ClientSideAuthenticator>>>();
        is_user_authenticated.set(authenticator().borrow().is_user_authenticated());
    });

    rsx! {
        div {class:"container",
            div {
                class: "relative -mx-4 flex items-center justify-between",

                // left side
                div { class: "w-60 max-w-full px-4 xl:mr-12 bold dark:text-white", "LOGO"}
                // center
                div { class:"flex w-full items-center justify-between px-4",
                    //center
                    div {
                        nav { id:"navbarCollapse", class: "navbar absolute right-0 z-30 w-[250px] rounded border-[.5px] border-body-color/50 bg-white px-6 py-4 duration-300 dark:border-body-color/20 dark:bg-dark lg:visible lg:static lg:w-auto lg:border-none lg:!bg-transparent lg:p-0 lg:opacity-100 visibility top-full opacity-100",
                            ul { class:"block lg:flex lg:space-x-12",
                                li { class:"group relative",
                                    Link { class: "flex py-2 text-base lg:mr-0 lg:inline-flex lg:px-0 lg:py-6 text-primary dark:text-white", to: Route::Home {}, "Home" }
                                }
                                li { class:"group relative",
                                    Link { class: "flex py-2 text-base lg:mr-0 lg:inline-flex lg:px-0 lg:py-6 text-primary dark:text-white", to: Route::Blog {id: 1}, "Blog" }                                }
                                li { class:"group relative",
                                    Link { class: "flex py-2 text-base lg:mr-0 lg:inline-flex lg:px-0 lg:py-6 text-primary dark:text-white", to: Route::Home {}, "About" }
                                }
                            }
                        }
                    }
                    //right
                    div { class: "flex items-center justify-end pr-16 lg:pr-0",
                        if is_user_authenticated() {
                           {"Hello, Mate"}
                        } else {
                            Link { class: "hidden px-7 py-3 text-base font-medium text-dark hover:opacity-70 dark:text-white md:block", to: Route::Login {}, "Sign In" }
                            Link { class: "ease-in-up shadow-btn hover:shadow-btn-hover hidden rounded-sm bg-primary px-8 py-3 text-base font-medium text-white transition duration-300 hover:bg-opacity-90 md:block md:px-9 lg:px-6 xl:px-9", to: Route::Registration {}, "Sign Up" }
                        }
                        div {
                            // ThemeToggle
                            DarkModeToggle{}
                        }
                    }
                }
            }
        }
    }
}
