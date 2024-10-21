use crate::route::Route;
use crate::rsx::dark_mode::DarkMode;
use argentum_rest_infrastructure::data_type::HttpParams;
use argentum_rest_infrastructure::data_type::HttpRequest;
use argentum_rest_infrastructure::data_type::{
    EmptyHeaderParams, EmptyQueryParams, EmptyRequestBody,
};
use argentum_user_account_rest::client::Client;
use argentum_user_account_rest::dto::operation_response_enum::AnonymousRegistersOperationResponseEnum;
use argentum_user_account_rest::dto::params::AnonymousRegistersParams;
use argentum_user_account_rest::dto::path_params::AnonymousRegistersPathParams;
use argentum_user_account_rest::dto::request::AnonymousRegistersRequest;
use argentum_user_account_rest::dto::response::AnonymousRegisteredSuccessfullyResponse::ApplicationJson;
use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use dioxus_sdk::storage::*;

pub(crate) fn App() -> Element {
    use_context_provider(|| Signal::new(DarkMode(false)));

    let mut token: Signal<Option<String>> = use_signal(|| None);

    use_future(move || async move {
        let mut local_storage_token =
            use_synced_storage::<LocalStorage, Option<String>>("x_auth_token".to_string(), || None);

        let local_storage_user_token = use_synced_storage::<LocalStorage, Option<String>>(
            "x_auth_user_token".to_string(),
            || None,
        );

        if local_storage_token().is_some() {
            token.set(local_storage_token());
        } else if local_storage_user_token().is_none() {
            let client = Client::new("http://localhost:8082".to_string(), "/api/v1".to_string());

            let req = AnonymousRegistersRequest::new(
                EmptyRequestBody {},
                AnonymousRegistersParams::new(
                    AnonymousRegistersPathParams::new(),
                    EmptyQueryParams {},
                    EmptyHeaderParams {},
                ),
            );
            let res = client.anonymous_registers(req).await;

            match res {
                Ok(data) => match data {
                    AnonymousRegistersOperationResponseEnum::Status201(r) => match r {
                        ApplicationJson(j) => {
                            local_storage_token.set(Some(j.0.token));
                            token.set(local_storage_token());
                        }
                    },
                },
                Err(_) => {
                    error!("Cant get token");
                }
            }
        }
    });

    rsx! {
        Router::<Route> {}
        div {
            match token.read().as_ref() {
                Some(t) => rsx!{ "Token: {t}"},
                None => rsx!{"No token"},
            }
        },
    }
}
