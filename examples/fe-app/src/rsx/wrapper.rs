use crate::route::Route;
use argentum_skeleton_ui::rsx::component::Footer;
use argentum_skeleton_ui::rsx::component::NavBar;
use argentum_standard_ui::rsx::component::dark_mode::DarkMode;
use argentum_standard_ui::rsx::component::dark_mode::ThemeConfig;
use argentum_user_account_ui::rsx::AuthBarProps;
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
            class: "flex flex-col min-h-screen justify-between items-center bg-base-100 dark:bg-base-900 shadow",
            "data-theme": theme_class(),
            header {
                class: "navbar bg-base-200 shadow-md",
                NavBar {
                    logo: rsx! { "LOGO" },
                    auth: AuthBarProps {
                        login_route: Route::Login {},
                        registration_route: Route::Registration {},
                        logout_redirect: Route::Home {},
                        links: vec![],
                    },
                    links: vec![
                        ("Home".into(), Route::Home {}),
                        ("Blog".into(), Route::Blog {id: 1}),
                        ("About".into(), Route::About {}),
                        ("Contacts".into(), Route::Contacts {}),
                    ],
                    theme_config: theme_config()
                }
            }
            main {
                class: "relative prose flex-grow p-6 w-full max-w-4xl",
                Outlet::<Route> {}
            }
            Footer {
                title: "Follow us",
                links: vec![
                    ("GitLab".into(), "https://gitlab.com/argentum-toolkit/argentum".into()),
                    ("GitHub".into(), "https://github.com/argentum-toolkit/argentum".into()),
                    ("LinkedIn".into(), "https://www.linkedin.com/in/vital-leshchyk".into()),
                ]
            }
        }
    }
}
