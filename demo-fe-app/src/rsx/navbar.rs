use crate::route::Route;
use crate::rsx::dark_mode::DarkModeToggle;
use dioxus::prelude::*;

#[component]
pub(crate) fn NavBar() -> Element {
    rsx! {
        nav {
            ul {
                li {
                    Link { to: Route::Home {}, "Home" }
                }
                li {
                    DarkModeToggle {}
                }
                li {
                    Link { to: Route::Registration {}, "Registration" }
                }
                li {
                    Link { to: Route::Login {}, "Login" }
                }
            }
        }
    }
}
