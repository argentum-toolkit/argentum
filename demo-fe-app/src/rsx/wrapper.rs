use crate::route::Route;
use crate::rsx::component::Footer;
use crate::rsx::component::NavBar;
use argentum_standard_ui::rsx::component::dark_mode::DarkMode;
use argentum_standard_ui::rsx::component::dark_mode::ThemeConfig;
use dioxus::prelude::*;

#[component]
pub(crate) fn Wrapper() -> Element {
    let theme_config = use_signal(|| ThemeConfig::new("dark".into(), "corporate".into()));

    let mut theme_class = use_signal(|| theme_config().dark_theme);
    use_effect(move || {
        theme_class.set(theme_config().mode_to_theme_name(use_context::<Signal<DarkMode>>()()));
    });

    rsx! {
        div {
            class: "flex flex-col min-h-screen bg-base-200 justify-between items-center bg-base-200 dark:bg-base-900 shadow",
            "data-theme": theme_class(),
            header {
                class: "navbar bg-base-100 shadow-md",
                NavBar { theme_config: theme_config() }
            }
            main {
                class: "prose flex-grow p-6",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}
