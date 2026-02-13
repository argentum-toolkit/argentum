use std::rc::Rc;

use argentum_user_rest::dto::response::Status404Response;
use dioxus::prelude::*;

use crate::security::rsx::AuthMenu;
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
use argentum_user_rest::dto::response::Status401Response;

#[derive(Clone, PartialEq, Props)]
pub struct AuthBarProps<R>
where
    R: Routable + std::cmp::PartialEq,
{
    pub login_route: R,
    pub registration_route: R,
    pub logout_redirect: R,
}

#[component]
pub fn AuthBar<R: Routable + std::cmp::PartialEq>(props: AuthBarProps<R>) -> Element {
    let mut first_name: Signal<Option<String>> = use_signal(|| None);
    use argentum_standard_ui::service::redirect;

    let logout_redirect = props.logout_redirect.clone();
    let logout_redirect_spawn = props.logout_redirect.clone();

    let mut user_loaded: Signal<bool> = use_signal(|| false);
    spawn(async move {
        if user_loaded() {
            //prevent multiple requests
            return;
        }

        user_loaded.set(true);

        use crate::security::ClientSideAuthenticator;
        use dioxus_logger::tracing::error;

        if first_name().is_some() {
            return;
        }

        let authenticator: Signal<ClientSideAuthenticator> = use_context();
        if let Some(user) = authenticator().get_user() {
            let client = use_context::<Signal<Rc<Client>>>();
            let req = GetUserRequest::new(
                EmptyRequestBody {},
                GetUserParams::new(
                    GetUserPathParams::new(user.1),
                    EmptyQueryParams {},
                    AuthHeaderParams::new(user.0),
                ),
            );

            let result = client.read().get_user(req).await;
            let first = match result {
                Ok(response) => {
                    match response {
                        GetUserOperationResponseEnum::Status200(r) => match r {
                            GetUserOkResponse::ApplicationJson(j) => Some(j.0.name.first.clone()),
                        },
                        GetUserOperationResponseEnum::Status401(r) => match r {
                            Status401Response::ApplicationProblemJson(j) => {
                                error!(
                                    "UNAUTHORIZED {}: {}",
                                    j.0.title,
                                    j.0.detail.unwrap_or_default()
                                );

                                //todo: show error in pop-up
                                authenticator().logout();
                                let _ = redirect(logout_redirect_spawn);

                                None
                            }
                        },
                        GetUserOperationResponseEnum::Status404(r) => match r {
                            Status404Response::ApplicationProblemJson(j) => {
                                error!(
                                    "NOT FOUND {}: {}",
                                    j.0.title,
                                    j.0.detail.unwrap_or_default()
                                );

                                //todo: show error in pop-up
                                authenticator().logout();
                                let _ = redirect(logout_redirect_spawn);

                                None
                            }
                        },
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

    if first_name().is_some() {
        return rsx! {
            AuthMenu {
                name: first_name().unwrap_or_else(|| "".to_string()),
                logout_redirect,
            }
        };
    }

    rsx! {
        Link { class: "btn btn-sm btn-primary p-1", to: props.login_route, "Sign In" }
        Link { class: "btn btn-sm btn-outline p-1", to: props.registration_route, "Sign Up" }
    }
}
