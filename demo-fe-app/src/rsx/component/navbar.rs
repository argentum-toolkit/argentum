use crate::route::Route;

use argentum_standard_ui::rsx::component::dark_mode::DarkModeToggle;
use dioxus::prelude::*;

#[component]
fn AuthBar() -> Element {
    let mut mounted = use_signal(|| false);

    #[cfg(feature = "web")]
    {
        use_effect(move || {
            mounted.set(true);
        });

        return rsx! {
            if mounted() {
                argentum_user_account_ui::security::rsx::AuthBar<Route> {
                    login_route: Route::Login {},
                    registration_route: Route::Registration {},
                    logout_redirect: Route::Home {},
                }
            }
        };
    }

    #[cfg(not(feature = "web"))]
    {
        return rsx! {"Loading..."};
    }
}

#[component]
pub(crate) fn NavBar() -> Element {
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
                        AuthBar {}
                        DarkModeToggle {}
                    }
                }
            }
        }
    }
}
