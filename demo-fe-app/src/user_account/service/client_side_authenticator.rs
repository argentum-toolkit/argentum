use std::str::FromStr;
use std::sync::Arc;

use dioxus::hooks::use_signal;
use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use svg_attributes::to;

use super::AnonymousRegistersHandlerTrait;

use web_sys::Storage;

type AuthHash = String;
type UserId = uuid::Uuid;

pub enum Security {
    Anonymous(AuthHash),
    Authenticated(AuthHash, UserId),
}

pub struct SecurityRepository {
    local_storage: Arc<Storage>,
}

impl SecurityRepository {
    pub fn new(local_storage: Arc<Storage>) -> Self {
        Self {local_storage}
    }

    pub fn clear(&self) {
        self.local_storage.delete("x_auth_token");//anonymous
        self.local_storage.delete("x_auth_user_token");
        self.local_storage.delete("x_auth_user_id");
    }

    pub fn read(&self) -> Option<Security> {
        let anonymous_token = self.local_storage.get_item("x_auth_token").unwrap_or(None);
        
        let user_token = self.local_storage.get_item("x_auth_user_token").unwrap_or(None);
        let user_id = self.local_storage.get_item("x_auth_user_id").unwrap_or(None).and_then(|id| uuid::Uuid::from_str(id.as_str()).ok());

        let authenticated = match (user_token.clone(), user_id) {
            (Some(token), Some(id)) => Some(Security::Authenticated(token, id)),
            _ => None,
        };

        
        if let Some(a) = authenticated {
            Some(a)
        } else if let Some(token) = anonymous_token {
            Some(Security::Anonymous(token))
        } else {
            self.clear();

            None
        }
    }

    pub fn save(&self, security: Security) {
        match security {
            Security::Anonymous(hash) => {
                self.local_storage.delete("x_auth_user_token");
                self.local_storage.delete("x_auth_user_id");
            },
            Security::Authenticated(hash, user_id) => {
                self.local_storage.delete("x_auth_token");//anonymous
            },
        };
    }
}

#[derive(Clone)]
#[cfg(feature = "web")]
pub struct ClientSideAuthenticator {
    security_repository: Arc<SecurityRepository>,
}

#[cfg(feature = "web")]
impl ClientSideAuthenticator {
    pub fn new(
        handler: Arc<dyn AnonymousRegistersHandlerTrait>,
        security_repository: Arc<SecurityRepository>,
    ) -> Self {
        let need_token = 
            use_signal(|| security_repository.read().is_none());

            // let neeeeed = security_repository.read().is_none();

        let sr = security_repository.clone();

        let callback = |token: String| async move {
            sr.save(Security::Anonymous(token));
        };

        let fetch_new = use_resource(move |callback: dyn Fn(String)| async move {
            error!("AAAAAAAAAAAAAAAAaa");
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
                                error!("HOHOHOHOHO");
                                // anonymous_token.set(Some(j.0.token.clone()));
                                // local_storage.set_item("x_auth_token", j.0.token.clone().as_str());
                            }
                        },
                    },
                    Err(e) => {
                        error!("Cant get token from server. Error: `{:?}`", e);
                    }
                }
            
        });

        spawn(async move {
            error!("TROLOLO");
            fetch_new(callback);
        });

        // let f: dyn FnMut() = {
        //     spawn(async move {
        //             use argentum_rest_infrastructure::data_type::HttpParams;
        //             use argentum_rest_infrastructure::data_type::HttpRequest;
        //             use argentum_rest_infrastructure::data_type::{
        //                 EmptyHeaderParams, EmptyQueryParams, EmptyRequestBody,
        //             };
                    
        //             use argentum_user_account_rest::client::Client;
        //             use argentum_user_account_rest::dto::operation_response_enum::AnonymousRegistersOperationResponseEnum;
        //             use argentum_user_account_rest::dto::params::AnonymousRegistersParams;
        //             use argentum_user_account_rest::dto::path_params::AnonymousRegistersPathParams;
        //             use argentum_user_account_rest::dto::request::AnonymousRegistersRequest;
        //             use argentum_user_account_rest::dto::response::AnonymousRegisteredSuccessfullyResponse::ApplicationJson;
                    
        //             if need_token() {


        //                 //TODO: remove hardcode
        //                 let client =
        //                     Client::new("http://localhost:8082".to_string(), "/api/v1".to_string());

        //                 let req = AnonymousRegistersRequest::new(
        //                     EmptyRequestBody {},
        //                     AnonymousRegistersParams::new(
        //                         AnonymousRegistersPathParams::new(),
        //                         EmptyQueryParams {},
        //                         EmptyHeaderParams {},
        //                     ),
        //                 );

        //                 let res: Result<AnonymousRegistersOperationResponseEnum, String> =
        //                     client.anonymous_registers(req).await;

        //                 match res {
        //                     Ok(data) => match data {
        //                         AnonymousRegistersOperationResponseEnum::Status201(r) => match r {
        //                             ApplicationJson(j) => {
        //                                 // anonymous_token.set(Some(j.0.token.clone()));
        //                                 // local_storage.set_item("x_auth_token", j.0.token.clone().as_str());
        //                             }
        //                         },
        //                     },
        //                     Err(e) => {
        //                         error!("Cant get token from server. Error: `{:?}`", e);
        //                     }
        //                 }
        //         };
        //     });

        // };

        // spawn(async move {
        //     f();
        // });


        // // spawn(async move {
        // use_effect(move || {
        //     // if need_token() {
        //     if sr.read().is_none() {
        //         match handler.execute() {
        //             Ok(token) => {
        //                 // sr.save(Security::Anonymous(token));
        //             },
        //             Err(e) => error!("Cant get token from server. Error: `{:?}`", e),
        //         }
        //     }
        // });

        Self { security_repository }
    }

    pub fn auth_user(&mut self, token: AuthHash, user_id: UserId) {
        self.security_repository.save(Security::Authenticated(token, user_id));
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
