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
            class: "{theme_class}",
            header { class: "navbar",
                NavBar {}
            }
            main {
                class: "dark:bg-gray-dark text-body-color dark:text-body-color-dark pt-16 md:pt-20 lg:pt-28",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}
