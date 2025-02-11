use std::sync::Arc;

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

use async_trait::async_trait;

use super::AnonymousRegistersHandlerTrait;

pub struct AnonymousRegistersHandler {
    user_account_client: Arc<Client>,
}

impl AnonymousRegistersHandler {
    pub fn new(user_account_client: Arc<Client>) -> Self {
        Self {
            user_account_client,
        }
    }
}

#[async_trait]
impl AnonymousRegistersHandlerTrait for AnonymousRegistersHandler {
    fn execute(&self) -> Result<String, String> {
        let req = AnonymousRegistersRequest::new(
            EmptyRequestBody {},
            AnonymousRegistersParams::new(
                AnonymousRegistersPathParams::new(),
                EmptyQueryParams {},
                EmptyHeaderParams {},
            ),
        );

        use futures::executor::block_on;
        block_on(async move {
            let res: Result<AnonymousRegistersOperationResponseEnum, String> =
                self.user_account_client.anonymous_registers(req).await;

            match res {
                Ok(data) => match data {
                    AnonymousRegistersOperationResponseEnum::Status201(r) => match r {
                        ApplicationJson(j) => Ok(j.0.token.to_string()),
                    },
                },
                Err(e) => Err(format!("Cant get token from server. Error: `{:?}`", e)),
            }
        })
    }
}


// use dioxus::prelude::*;
// pub fn create_auth_anonymous_handler(
//     // props: UserLoginsWithPasswordProps,
// ) -> impl FnMut() -> Result<String, String> {

//     let handler = {
//         move ||  {
//             spawn(async move {
//                 let req = AnonymousRegistersRequest::new(
//                     EmptyRequestBody {},
//                     AnonymousRegistersParams::new(
//                         AnonymousRegistersPathParams::new(),
//                         EmptyQueryParams {},
//                         EmptyHeaderParams {},
//                     ),
//                 );
        
//                 // use futures::executor::block_on;
//                 // block_on(async move {
//                     let res: Result<AnonymousRegistersOperationResponseEnum, String> =
//                         user_account_client.anonymous_registers(req).await;
        
//                     match res {
//                         Ok(data) => match data {
//                             AnonymousRegistersOperationResponseEnum::Status201(r) => match r {
//                                 ApplicationJson(j) => Ok(j.0.token.to_string()),
//                             },
//                         },
//                         Err(e) => Err(format!("Cant get token from server. Error: `{:?}`", e)),
//                     }
//                 // })
//             });
//         }
//     };

//     handler
// }