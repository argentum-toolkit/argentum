use std::sync::Arc;

use super::AuthHash;
use super::Security;
use super::SecurityRepository;
use super::UserId;

use argentum_user_account_rest::client::Client;

use dioxus::hooks::use_signal;
use dioxus::prelude::*;
use dioxus_logger::tracing::error;

#[derive(Clone)]
pub struct ClientSideAuthenticator {
    client: Arc<Client>,
    security_repository: Arc<SecurityRepository>,
}

impl ClientSideAuthenticator {
    pub fn new(client: Arc<Client>, security_repository: Arc<SecurityRepository>) -> Self {
        Self {
            client,
            security_repository,
        }
    }

    pub async fn init(&self) {
        if self.security_repository.read().is_none() {
            let res = self.fetch_new_anonymous_token().await;

            match res.clone() {
                Ok(token) => {
                    self.security_repository.save(Security::Anonymous(token));
                }
                _ => todo!(),
            };
        }
    }

    pub async fn fetch_new_anonymous_token(
        &self,
    ) -> Result<std::string::String, std::string::String> {
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

        let req = AnonymousRegistersRequest::new(
            EmptyRequestBody {},
            AnonymousRegistersParams::new(
                AnonymousRegistersPathParams::new(),
                EmptyQueryParams {},
                EmptyHeaderParams {},
            ),
        );

        let res: Result<AnonymousRegistersOperationResponseEnum, String> =
            self.client.anonymous_registers(req).await;

        match res {
            Ok(data) => match data {
                AnonymousRegistersOperationResponseEnum::Status201(r) => match r {
                    ApplicationJson(j) => Ok(j.0.token.clone()),
                },
            },
            Err(e) => Err(format!("Cant get token from server. Error: `{:?}`", e)),
        }
    }

    pub fn auth_user(&mut self, token: AuthHash, user_id: UserId) {
        self.security_repository
            .save(Security::Authenticated(token, user_id));
    }

    pub fn logout(&mut self) {
        self.security_repository.clear();
    }

    pub fn anonymous_token(&self) -> Option<AuthHash> {
        match self.security_repository.read() {
            Some(Security::Anonymous(token)) => Some(token),
            _ => None,
        }
    }

    pub fn user(&self) -> Option<(AuthHash, UserId)> {
        match self.security_repository.read() {
            Some(Security::Authenticated(token, id)) => Some((token, id)),
            _ => None,
        }
    }
}
