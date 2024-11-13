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
        let mut local_storage_anonymous_token =
            use_synced_storage::<LocalStorage, Option<String>>("x_auth_token".to_string(), || None);

        let mut local_storage_user_token = use_synced_storage::<LocalStorage, Option<String>>(
            "x_auth_user_token".to_string(),
            || None,
        );

        let mut local_storage_user_id = use_synced_storage::<LocalStorage, Option<uuid::Uuid>>(
            "x_auth_user_id".to_string(),
            || None,
        );

        let mut anonymous_token = use_signal(|| local_storage_anonymous_token());
        let mut user_token = use_signal(|| local_storage_user_token());
        let mut user_id = use_signal(|| local_storage_user_id());

        if anonymous_token().is_some() && user_token().is_some() {
            //Invalid state. Clearing.
            // TODO: logout; go to main page
            anonymous_token.set(None);
            user_token.set(None);
            user_id.set(None);
            local_storage_anonymous_token.set(None);
            local_storage_user_token.set(None);
            local_storage_user_id.set(None);
        }

        spawn(async move {
            if user_token().is_some() {
            } else if anonymous_token().is_none() {
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

                let res = client.anonymous_registers(req).await;

                match res {
                    Ok(data) => match data {
                        AnonymousRegistersOperationResponseEnum::Status201(r) => match r {
                            ApplicationJson(j) => {
                                local_storage_anonymous_token.set(Some(j.0.token));
                                anonymous_token.set(local_storage_anonymous_token());
                            }
                        },
                    },
                    Err(e) => {
                        error!("Cant get token from server. Error: `{:?}`", e);
                    }
                }
            }
        });

        Self {
            anonymous_token: anonymous_token(),
            user_token: user_token(),
            user_id: user_id(),
        }
    }

    pub fn auth_anonymous(&mut self, token: String) {
        self.user_token = None;
        self.user_id = None;
        self.anonymous_token = Some(token);
    }

    pub fn auth_user(&mut self, token: String, user_id: uuid::Uuid) {
        self.anonymous_token = None;
        self.user_token = Some(token.clone());
        self.user_id = Some(user_id.clone());

        let mut local_storage_anonymous_token =
            use_synced_storage::<LocalStorage, Option<String>>("x_auth_token".to_string(), || None);

        local_storage_anonymous_token.set(None);

        let mut local_storage_user_token = use_synced_storage::<LocalStorage, Option<String>>(
            "x_auth_user_token".to_string(),
            || None,
        );

        local_storage_user_token.set(Some(token));

        let mut local_storage_user_id = use_synced_storage::<LocalStorage, Option<uuid::Uuid>>(
            "x_auth_user_id".to_string(),
            || None,
        );

        local_storage_user_id.set(Some(user_id.clone()));
    }

    pub fn logout(&mut self) {
        self.anonymous_token = None;
        self.user_token = None;
        self.user_id = None;

        let mut local_storage_anonymous_token =
            use_synced_storage::<LocalStorage, Option<String>>("x_auth_token".to_string(), || None);

        local_storage_anonymous_token.set(None);

        let mut local_storage_user_token = use_synced_storage::<LocalStorage, Option<String>>(
            "x_auth_user_token".to_string(),
            || None,
        );

        local_storage_user_token.set(None);

        let mut local_storage_user_id = use_synced_storage::<LocalStorage, Option<uuid::Uuid>>(
            "x_auth_user_id".to_string(),
            || None,
        );

        local_storage_user_id.set(None);
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
