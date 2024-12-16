use std::str::FromStr;

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
use dioxus::hooks::use_signal;
use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

#[derive(Clone)]
#[cfg(feature = "web")]
pub struct ClientSideAuthenticator {
    anonymous_token: Option<String>,
    user_token: Option<String>,
    user_id: Option<uuid::Uuid>, //TODO: uuid
}

#[cfg(feature = "web")]
impl ClientSideAuthenticator {
    pub fn new() -> Self {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let mut anonymous_token =
            use_signal(|| local_storage.get_item("x_auth_token").unwrap_or(None));
        let mut user_token =
            use_signal(|| local_storage.get_item("x_auth_user_token").unwrap_or(None));
        let mut user_id = use_signal(|| local_storage.get_item("x_auth_user_id").unwrap_or(None));

        if anonymous_token().is_some() && user_token().is_some() {
            //Invalid state. Clearing.
            // TODO: logout; go to main page
            anonymous_token.set(None);
            user_token.set(None);
            user_id.set(None);
        }

        spawn(async move {
            if user_token().is_some() {
            } else if anonymous_token().is_none() {
                //TODO: remove hardcode
                let client =
                    Client::new("http://localhost:8082".to_string(), "/api/v1".to_string());

                let req = AnonymousRegistersRequest::new(
                    EmptyRequestBody {},
                    AnonymousRegistersParams::new(
                        AnonymousRegistersPathParams::new(),
                        EmptyQueryParams {},
                        EmptyHeaderParams {},
                    ),
                );

                let res: Result<AnonymousRegistersOperationResponseEnum, String> =
                    client.anonymous_registers(req).await;

                match res {
                    Ok(data) => match data {
                        AnonymousRegistersOperationResponseEnum::Status201(r) => match r {
                            ApplicationJson(j) => {
                                anonymous_token.set(Some(j.0.token.clone()));
                                local_storage.set_item("x_auth_token", j.0.token.clone().as_str());
                            }
                        },
                    },
                    Err(e) => {
                        error!("Cant get token from server. Error: `{:?}`", e);
                    }
                }
            }
        });

        let id = match user_id() {
            Some(s) => Some(uuid::Uuid::from_str(s.as_str()).unwrap()),
            None => None,
        };

        Self {
            anonymous_token: anonymous_token(),
            user_token: user_token(),
            user_id: id,
        }
    }

    pub fn auth_anonymous(&mut self, token: String) {
        self.user_token = None;
        self.user_id = None;
        self.anonymous_token = Some(token);
    }

    pub fn auth_user(&mut self, token: String, user_id: uuid::Uuid) {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        self.anonymous_token = None;
        self.user_token = Some(token.clone());
        self.user_id = Some(user_id.clone());

        local_storage.delete("x_auth_token");
        local_storage.set_item("x_auth_user_token", token.as_str());
        local_storage.set_item("x_auth_user_id", user_id.to_string().as_str());
    }

    pub fn logout(&mut self) {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        self.anonymous_token = None;
        self.user_token = None;
        self.user_id = None;

        local_storage.delete("x_auth_token");
        local_storage.delete("x_auth_user_token");
        local_storage.delete("x_auth_user_id");
    }

    pub fn anonymous_token(&self) -> Option<String> {
        self.anonymous_token.clone()
    }

    pub fn user_token(&self) -> Option<String> {
        self.user_token.clone()
    }

    pub fn user_id(&self) -> Option<uuid::Uuid> {
        self.user_id.clone()
    }

    pub fn is_user_authenticated(&self) -> bool {
        self.user_token.is_some()
    }

    pub fn is_anonymous_authenticated(&self) -> bool {
        self.anonymous_token.is_some()
    }
}

impl Default for ClientSideAuthenticator {
    fn default() -> Self {
        Self::new()
    }
}
