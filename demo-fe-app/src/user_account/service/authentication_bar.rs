
use crate::route::Route;

use dioxus::prelude::*;

#[component]
pub fn AuthenticationBar() -> Element {
    #[cfg(feature = "web")]
    {
        let mut first_name: Signal<Option<String>> = use_signal(|| None);

        spawn(async move {
            use crate::user_account::service::ClientSideAuthenticator;
            use dioxus_logger::tracing::error;
            let authenticator: Signal<ClientSideAuthenticator> = use_context();
            if authenticator().is_user_authenticated() && first_name().is_none() {
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

                let user_id = authenticator().user_id();
                let user_token = authenticator().user_token();

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
                {"Hello, " } {first_name.unwrap()}
            };
        }
    }

    rsx! {
        Link { class: "hidden px-7 py-3 text-base font-medium text-dark hover:opacity-70 dark:text-white md:block", to: Route::Login {}, "Sign In" }
        Link { class: "ease-in-up shadow-btn hover:shadow-btn-hover hidden rounded-sm bg-primary px-8 py-3 text-base font-medium text-white transition duration-300 hover:bg-opacity-90 md:block md:px-9 lg:px-6 xl:px-9", to: Route::Registration {}, "Sign Up" }
    }

}
