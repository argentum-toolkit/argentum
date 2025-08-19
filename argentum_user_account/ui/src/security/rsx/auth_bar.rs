use std::rc::Rc;

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

    spawn(async move {
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
                logout_redirect: props.logout_redirect,
            }
        };
    }

    rsx! {
        Link { class: "btn btn-sm btn-primary p-1", to: props.login_route, "Sign In" }
        Link { class: "btn btn-sm btn-outline p-1", to: props.registration_route, "Sign Up" }
    }
}
