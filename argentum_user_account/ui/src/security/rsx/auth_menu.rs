use dioxus::prelude::*;

#[component]
pub(crate) fn AuthMenu(name: String) -> Element {
    use crate::security::ClientSideAuthenticator;
    use argentum_standard_ui::service::redirect;
    let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

    let mut hidden = use_signal(|| "hidden");

    rsx! {
        div {
            class: "relative text-left",
            div {
                class:"whitespace-nowrap",
                div {
                    onclick: move |_event| {
                        let h = match hidden() {
                            "hidden" => "",
                            _ => "hidden",
                        };

                        hidden.set(h);
                    },
                    "type": "button",
                    class: "flex whitespace-nowrap text-md pe-1 font-medium text-gray-900 hover:text-blue-600 dark:hover:text-blue-400 md:me-0 focus:ring-4 focus:ring-gray-600 dark:focus:ring-gray-400 dark:text-white",
                    id: "menu-button", "aria-expanded": "true", "aria-haspopup": "true",
                    "Hello, {name}",
                    svg {
                        class:"w-2.5 h-7 ms-3", "aria-hidden": "true", xmlns: "http://www.w3.org/2000/svg", fill: "none", "viewBox": "0 0 10 6",
                        path {
                            stroke: "currentColor", "stroke-linecap": "round", "stroke-linejoin": "round", "stroke-width": "2", d: "m1 1 4 4 4-4",
                        }
                    }
                }
            }
            div {
                class: "{hidden} absolute right-0 z-1 mt-0 origin-top-right rounded-sm bg-white shadow-lg ring-1 ring-black/5 focus:outline-none",
                role: "menu",
                "aria-orientation":"vertical",
                "aria-labelledby":"menu-button",
                "tabindex":"-1",
                div {
                    class: "py-1", role: "none",
                    link {
                        onclick: move |_| {
                            authenticator().logout();
                            //TODO: enable redirect
                            // redirect(Route::Home {});
                        },
                        href: "javascript:void(0)",
                        class: "whitespace-nowrap block px-4 py-2 text-sm text-gray-700",
                        role:"menuitem",
                        tabindex:"-1",
                        id: "menu-item-0",
                        "Sign out",
                    }
                }
            }
        }
    }
}
