use crate::route::Route;

use argentum_standard_ui::rsx::component::dark_mode::{DarkModeToggle, ThemeConfig};
use dioxus::prelude::*;

#[component]
fn AuthBar() -> Element {
    #[cfg(feature = "web")]
    {
        let mut mounted = use_signal(|| false);

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
pub(crate) fn NavBar(theme_config: ThemeConfig) -> Element {
    rsx! {
        div { class: "w-30 max-w-full px-4 xl:mr-2 bold dark:text-white", "LOGO"}

        nav {
            class: "flex space-x-6 text-base-content",
            Link { class: "hover:text-primary", to: Route::Home {}, "Home" }
            Link { class: "hover:text-primary", to: Route::Blog { id: 1 }, "Blog" }
            Link { class: "hover:text-primary", to: Route::Home {}, "About" }
        }

        div {
            class: "flex items-center space-x-4 ml-auto whitespace-nowrap",
            AuthBar {}
            DarkModeToggle { theme_config }
        }
    }
}
