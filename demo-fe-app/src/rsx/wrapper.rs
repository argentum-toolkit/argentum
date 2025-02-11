use crate::route::Route;
use crate::rsx::footer::Footer;
use crate::rsx::navbar::NavBar;
use dioxus::prelude::*;

#[component]
pub(crate) fn Wrapper() -> Element {
    let mut theme_class = use_signal(|| "");

    #[cfg(feature = "web")]
    use_effect(move || {
        use super::dark_mode::DarkMode;

        let dark_mode = use_context::<Signal<DarkMode>>();

        let class = match dark_mode().0 {
            true => "dark",
            false => "",
        };

        theme_class.set(class);
    });

    #[cfg(not(feature = "web"))]
    use_effect(move || {
        theme_class.set("");
    });

    rsx! {
        div {
            class: "{theme_class}",
            header { class: "header left-0 top-0 z-40 flex w-full items-center dark:bg-gray-dark dark:shadow-sticky-dark fixed z-[9999] bg-white !bg-opacity-80 shadow-sticky backdrop-blur-sm transition",
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
