use argentum_standard_ui::rsx::component::dark_mode::{DarkModeToggle, ThemeConfig};
use argentum_user_account_ui::rsx::{AuthBar, AuthBarProps};

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct NavBarProps<R>
where
    R: Routable + std::cmp::PartialEq,
{
    pub auth: AuthBarProps<R>,

    pub links: Vec<(String, R)>,
    pub theme_config: ThemeConfig,
}

#[component]
pub fn NavBar<R: Routable + std::cmp::PartialEq>(props: NavBarProps<R>) -> Element {
    rsx! {
        div { class: "w-30 max-w-full px-4 xl:mr-2 bold dark:text-white", "LOGO"}

        nav {
            class: "flex space-x-6 text-base-content",

            for (label, route) in props.links.iter() {
                Link { class: "hover:text-primary", to: route.clone(), "{label}" }
            }
        }

        div {
            class: "flex items-center space-x-4 ml-auto whitespace-nowrap",
            AuthBar<R> {
                    ..props.auth
                }
            DarkModeToggle { theme_config: props.theme_config }
        }
    }
}
