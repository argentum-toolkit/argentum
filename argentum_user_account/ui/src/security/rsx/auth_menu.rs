use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AuthMenuProps<R>
where
    R: Routable + std::cmp::PartialEq,
{
    pub name: String,
    pub logout_redirect: R,
}

#[component]
pub(crate) fn AuthMenu<R: Routable + std::cmp::PartialEq>(props: AuthMenuProps<R>) -> Element {
    use crate::security::ClientSideAuthenticator;
    use argentum_standard_ui::service::redirect;
    let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

    rsx! {
        div {
            class: "dropdown dropdown-hover",
            div {
                class: "m-1",
                tabindex: "0",
                role: "button",
                    "Hello, {props.name}"
            }
            ul {
                class: "dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm",
                tabindex: 0,
                li {
                    link {
                        onclick: move |_| {
                            authenticator().logout();
                            redirect(props.logout_redirect.clone());
                        },
                        href: "javascript:void(0)",
                        "Sign out",
                    }
                }
            }
        }
    }
}
