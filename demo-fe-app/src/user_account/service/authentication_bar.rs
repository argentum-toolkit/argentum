use crate::route::Route;

use dioxus::prelude::*;

#[component]
#[cfg(feature = "web")]
fn Menu(name: String) -> Element {
    use crate::user_account::service::ClientSideAuthenticator;
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
                            redirect(Route::Home {});
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

#[component]
pub fn AuthenticationBar() -> Element {
    // #[cfg(feature = "web")]
    // {
    //     let mut first_name: Signal<Option<String>> = use_signal(|| None);

    //     spawn(async move {
    //         use crate::user_account::service::ClientSideAuthenticator;
    //         use dioxus_logger::tracing::error;

    //         if first_name().is_some() {
    //             return ;
    //         }

    //         let authenticator: Signal<ClientSideAuthenticator> = use_context();
    //         if let Some(user) = authenticator().user() {
    //             use argentum_rest_infrastructure::data_type::HttpParams;
    //             use argentum_rest_infrastructure::data_type::HttpRequest;
    //             use argentum_rest_infrastructure::data_type::{
    //                 AuthHeaderParams, EmptyQueryParams, EmptyRequestBody,
    //             };
    //             use argentum_user_rest::client::Client;
    //             use argentum_user_rest::dto::operation_response_enum::GetUserOperationResponseEnum;
    //             use argentum_user_rest::dto::params::GetUserParams;
    //             use argentum_user_rest::dto::path_params::GetUserPathParams;
    //             use argentum_user_rest::dto::request::GetUserRequest;
    //             use argentum_user_rest::dto::response::GetUserOkResponse;

    //             let client =
    //                 Client::new("http://localhost:8082".to_string(), "/api/v1".to_string());
    //             let req = GetUserRequest::new(
    //                 EmptyRequestBody {},
    //                 GetUserParams::new(
    //                     GetUserPathParams::new(user.1),
    //                     EmptyQueryParams {},
    //                     AuthHeaderParams::new(user.0),
    //                 ),
    //             );

    //             let result = client.get_user(req).await;
    //             let first = match result {
    //                 Ok(response) => {
    //                     match response {
    //                         GetUserOperationResponseEnum::Status200(r) => match r {
    //                             GetUserOkResponse::ApplicationJson(j) => {
    //                                 Some(j.0.name.first.clone())
    //                             }
    //                         },
    //                         _en => {
    //                             //TODO: process other statuses
    //                             error!("Bad data. Error: ?????");
    //                             None
    //                         }
    //                     }
    //                 }
    //                 Err(e) => {
    //                     error!("Cant get user data from server. Error: `{:?}`", e);
    //                     None
    //                 }
    //             };

    //             first_name.set(first);
    //         };
    //     });

    //     if first_name().is_some() {
    //         return rsx! {
    //             Menu{name: first_name().unwrap_or_else(|| "".to_string())}
    //         };
    //     }
    // }

    rsx! {
        Link { class: "hidden px-7 py-3 text-base font-medium text-dark hover:opacity-70 dark:text-white md:block", to: Route::Login {}, "Sign In" }
        Link { class: "ease-in-up shadow-btn hover:shadow-btn-hover hidden rounded-sm bg-primary px-8 py-3 text-base font-medium text-white transition duration-300 hover:bg-opacity-90 md:block md:px-9 lg:px-6 xl:px-9", to: Route::Registration {}, "Sign Up" }
    }
}
