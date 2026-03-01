use std::rc::Rc;

use super::AuthHash;
use super::Security;
use super::SecurityRepository;
use super::UserId;

use argentum_user_account_rest::client::Client;

#[derive(Clone)]
pub struct ClientSideAuthenticator {
    client: Rc<Client>,
    security_repository: Rc<SecurityRepository>,
}

impl ClientSideAuthenticator {
    pub fn new(client: Rc<Client>, security_repository: Rc<SecurityRepository>) -> Self {
        Self {
            client,
            security_repository,
        }
    }

    pub async fn init(&self) {
        if self.security_repository.read().is_none() {
            let res = self.fetch_new_guest_token().await;

            match res {
                Ok(token) => {
                    self.security_repository.save(Security::Guest(token));
                }
                _ => todo!(),
            };
        }
    }

    pub async fn fetch_new_guest_token(&self) -> Result<std::string::String, std::string::String> {
        use argentum_rest_infrastructure::data_type::HttpParams;
        use argentum_rest_infrastructure::data_type::HttpRequest;
        use argentum_rest_infrastructure::data_type::{
            EmptyHeaderParams, EmptyQueryParams, EmptyRequestBody,
        };

        use argentum_user_account_rest::dto::operation_response_enum::GuestRegistersOperationResponseEnum;
        use argentum_user_account_rest::dto::params::GuestRegistersParams;
        use argentum_user_account_rest::dto::path_params::GuestRegistersPathParams;
        use argentum_user_account_rest::dto::request::GuestRegistersRequest;
        use argentum_user_account_rest::dto::response::GuestRegisteredSuccessfullyResponse::ApplicationJson;

        let req = GuestRegistersRequest::new(
            EmptyRequestBody {},
            GuestRegistersParams::new(
                GuestRegistersPathParams::new(),
                EmptyQueryParams {},
                EmptyHeaderParams {},
            ),
        );

        let res: Result<GuestRegistersOperationResponseEnum, String> =
            self.client.guest_registers(req).await;

        match res {
            Ok(data) => match data {
                GuestRegistersOperationResponseEnum::Status201(r) => match r {
                    ApplicationJson(j) => Ok(j.0.token.clone()),
                },
            },
            Err(e) => Err(format!("Cant get token from server. Error: `{:?}`", e)),
        }
    }

    pub fn auth_user(&mut self, token: AuthHash, user_id: UserId) {
        self.security_repository
            .save(Security::Member(token, user_id));
    }

    pub fn logout(&mut self) {
        self.security_repository.clear();
    }

    pub fn get_token(&self) -> Option<AuthHash> {
        match self.security_repository.read() {
            Some(Security::Member(token, _)) => Some(token),
            Some(Security::Guest(token)) => Some(token),
            _ => None,
        }
    }

    pub fn get_user(&self) -> Option<(AuthHash, UserId)> {
        match self.security_repository.read() {
            Some(Security::Member(token, id)) => Some((token, id)),
            _ => None,
        }
    }
}
