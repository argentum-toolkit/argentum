use crate::route::Route;
use crate::rsx::dark_mode::DarkModeToggle;

use dioxus::prelude::*;
use dioxus_logger::tracing::error;

#[component]
pub(crate) fn NavBar() -> Element {
    let mut first_name: Signal<Option<String>> = use_signal(|| None);


    #[cfg(feature = "web")]
    spawn(async move {
        use crate::user_account::service::ClientSideAuthenticator;
        use std::cell::RefCell;
        let authenticator: Signal<RefCell<ClientSideAuthenticator>> = use_context();
        if authenticator().borrow().is_user_authenticated() && first_name().is_none() {
            use argentum_rest_infrastructure::data_type::HttpParams;
            use argentum_rest_infrastructure::data_type::HttpRequest;
            use argentum_rest_infrastructure::data_type::{
                AuthHeaderParams, EmptyQueryParams, EmptyRequestBody,
            };
            use argentum_user_rest::client::Client;
            use argentum_user_rest::dto::operation_response_enum::GetUserOperationResponseEnum;
            use argentum_user_rest::dto::params::GetUserParams;
            use argentum_user_rest::dto::path_params::GetUserPathParams;
            use argentum_user_rest::dto::request::GetUserRequest;
            use argentum_user_rest::dto::response::GetUserOkResponse;

            let user_id = authenticator().borrow().user_id();
            let user_token = authenticator().borrow().user_token();

            let client = Client::new("http://localhost:8082".to_string(), "/api/v1".to_string());
            let req = GetUserRequest::new(
                EmptyRequestBody {},
                GetUserParams::new(
                    GetUserPathParams::new(user_id.unwrap()),
                    EmptyQueryParams {},
                    AuthHeaderParams::new(user_token.unwrap()),
                ),
            );

            let result = client.get_user(req).await;
            let first = match result {
                Ok(response) => {
                    match response {
                        GetUserOperationResponseEnum::Status200(r) => {
                            match r {
                                GetUserOkResponse::ApplicationJson(j) => {
                                    Some(j.0.name.first.clone())
                                }
                            }
                        }
                        _en => {
                            //TODO: process other statuses
                            error!("Bad data. Error: ?????");
                            None
                        }
                    }
                }
                Err(e) => {
                    error!("Cant get user data from server. Error: `{:?}`", e);
                    None
                }
            };

            first_name.set(first);
        };
    });

    rsx! {
        div {class:"container",
            div {
                class: "relative -mx-4 flex items-center justify-between",

                // left side
                div { class: "w-60 max-w-full px-4 xl:mr-12 bold dark:text-white", "LOGO"}
                // center
                div { class:"flex w-full items-center justify-between px-4",
                    //center
                    div {
                        nav { id:"navbarCollapse", class: "navbar absolute right-0 z-30 w-[250px] rounded border-[.5px] border-body-color/50 bg-white px-6 py-4 duration-300 dark:border-body-color/20 dark:bg-dark lg:visible lg:static lg:w-auto lg:border-none lg:!bg-transparent lg:p-0 lg:opacity-100 visibility top-full opacity-100",
                            ul { class:"block lg:flex lg:space-x-12",
                                li { class:"group relative",
                                    Link { class: "flex py-2 text-base lg:mr-0 lg:inline-flex lg:px-0 lg:py-6 text-primary dark:text-white", to: Route::Home {}, "Home" }
                                }
                                li { class:"group relative",
                                    Link { class: "flex py-2 text-base lg:mr-0 lg:inline-flex lg:px-0 lg:py-6 text-primary dark:text-white", to: Route::Blog {id: 1}, "Blog" }                                }
                                li { class:"group relative",
                                    Link { class: "flex py-2 text-base lg:mr-0 lg:inline-flex lg:px-0 lg:py-6 text-primary dark:text-white", to: Route::Home {}, "About" }
                                }
                            }
                        }
                    }
                    //right
                    div { class: "flex items-center justify-end pr-16 lg:pr-0",
                        if first_name().is_some() {
                           {"Hello, " } {first_name.unwrap()}
                        } else {
                            Link { class: "hidden px-7 py-3 text-base font-medium text-dark hover:opacity-70 dark:text-white md:block", to: Route::Login {}, "Sign In" }
                            Link { class: "ease-in-up shadow-btn hover:shadow-btn-hover hidden rounded-sm bg-primary px-8 py-3 text-base font-medium text-white transition duration-300 hover:bg-opacity-90 md:block md:px-9 lg:px-6 xl:px-9", to: Route::Registration {}, "Sign Up" }
                        }
                        div {
                            // ThemeToggle
                            DarkModeToggle{}
                        }
                    }
                }
            }
        }
    }
}
