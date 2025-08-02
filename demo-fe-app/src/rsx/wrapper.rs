use crate::route::Route;
use crate::rsx::component::Footer;
use crate::rsx::component::NavBar;
use argentum_standard_ui::rsx::component::dark_mode::DarkMode;
use dioxus::prelude::*;

#[component]
pub(crate) fn Wrapper() -> Element {
    let mut theme_class = use_signal(|| "".to_string());
    use_effect(move || {
        theme_class.set(use_context::<Signal<DarkMode>>()().to_string());
    });

    rsx! {
        div {
            class: "{theme_class} flex flex-col min-h-screen bg-base-200 justify-between items-center bg-base-200 dark:bg-base-900 shadow",
            "data-theme": "dracula",
            header {
                class: "navbar bg-base-100 shadow-md",
                NavBar {}
            }
            main {
                class: "prose flex-grow p-6",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}
