use dioxus::prelude::*;

#[cfg(feature = "web")]
const STORAGE_KEY: &str = "dark_mode_enabled";

#[derive(Clone, Copy)]
pub struct DarkMode(pub bool);

#[derive(PartialEq, Clone)]
pub struct ThemeConfig {
    pub dark_theme: String,
    pub light_theme: String,
}

impl ThemeConfig {
    pub fn new(dark_theme: String, light_theme: String) -> Self {
        Self {
            dark_theme,
            light_theme,
        }
    }

    pub fn mode_to_theme_name(&self, mode: DarkMode) -> String {
        if mode.0 {
            self.dark_theme.clone()
        } else {
            self.light_theme.clone()
        }
    }
}

#[cfg(feature = "web")]
fn extract_bool_or_false(key: &str) -> bool {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

    match local_storage.get_item(key) {
        Ok(Some(s)) => s.len() > 0 && s != "false",
        Ok(None) => false,
        Err(_) => false,
    }
}

pub fn use_dark_mode() {
    #[cfg(feature = "web")]
    {
        let is_dark = use_signal(|| extract_bool_or_false(STORAGE_KEY));

        use_context_provider(|| Signal::new(DarkMode(is_dark())));
    }
}

#[component]
pub fn DarkModeToggle(theme_config: ThemeConfig) -> Element {
    let mut dark_mode = use_signal(|| DarkMode(true));
    use_effect(move || {
        dark_mode.set(use_context::<Signal<DarkMode>>()());
    });

    rsx! {
        label {
            class: "swap swap-rotate items-center ",
            //this hidden checkbox controls the state
            input {
                type: "checkbox",
                class: "theme-controller",
                checked: dark_mode().0,
                onclick: move |_event| {
                    #[cfg(feature = "web")]
                    {
                        let mut dark_mode_context = use_context::<Signal<DarkMode>>();
                        let is_dark = !dark_mode_context().0;

                        dark_mode_context.set(DarkMode(is_dark));
                        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
                        let s = if is_dark {
                            "true"
                        } else {
                            "false"
                        };

                        local_storage.set_item(STORAGE_KEY, s);
                    }
                },
            }

            //sun icon
            svg {
                class: "swap-off h-7 w-7 fill-current",
                xmlns: "http://www.w3.org/2000/svg",
                "viewBox": "0 0 24 24",
                path {
                    "strokeWidth":"2",
                    "strokeLinecap":"round",
                    "strokeLinejoi":"round",
                    d: "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"
                }
            }

            //moon icon
            svg {
                class: "swap-on h-7 w-7 fill-current",
                xmlns: "http://www.w3.org/2000/svg",
                "viewBox": "0 0 24 24",
                path {
                    "strokeWidth":"2",
                    "strokeLinecap":"round",
                    "strokeLinejoi":"round",
                    d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"
                }
            }
        }
    }
}
