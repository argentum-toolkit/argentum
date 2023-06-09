use crate::api::dto::request::{
    AnonymousWithTokenChangesPasswordRequest,
};
use crate::api::server::handler::{
    AnonymousWithTokenChangesPasswordTrait,
};
use crate::rest::transformer::{ DtoToAnonymousWithTokenChangesPasswordParams};
use argentum_rest_infrastructure::data_type::error::{ HttpError, InternalServerError, UnprocessableEntity};
use argentum_rest_infrastructure::data_type::HttpResponse;
use argentum_user_account_api::models::EmptyResponse;
use argentum_user_account_business::use_case::restore_password::error::RestorePasswordError;
use argentum_user_business::entity::user::User;
use hyper::StatusCode;
use std::sync::Arc;
use argentum_user_account_business::use_case::restore_password::anonymous_with_token_changes_password::AnonymousWithTokenChangesPassword;

pub struct AnonymousWithTokenChangesPasswordHandler {
    uc: Arc<AnonymousWithTokenChangesPassword>,
    dto_to_anonymous_with_token_changes_password_params:
        Arc<DtoToAnonymousWithTokenChangesPasswordParams>,
}

impl AnonymousWithTokenChangesPasswordHandler {
    pub fn new(
        uc: Arc<AnonymousWithTokenChangesPassword>,
        dto_to_anonymous_with_token_changes_password_params: Arc<
            DtoToAnonymousWithTokenChangesPasswordParams,
        >,
    ) -> Self {
        AnonymousWithTokenChangesPasswordHandler {
            uc,
            dto_to_anonymous_with_token_changes_password_params,
        }
    }
}

impl AnonymousWithTokenChangesPasswordTrait for AnonymousWithTokenChangesPasswordHandler {
    fn handle(
        &self,
        req: AnonymousWithTokenChangesPasswordRequest,
        _user: User,
    ) -> Result<HttpResponse, HttpError> {
        let (token, pw) = self
            .dto_to_anonymous_with_token_changes_password_params
            .transform(req)?;

        let result = self.uc.execute(token, pw);

        match result {
            Ok(_) => {
                let dto = EmptyResponse::from(serde_json::Value::Null);

                Ok(HttpResponse::new(StatusCode::OK, Box::new(dto)))
            }
            Err(e) => match e {
                RestorePasswordError::TokenExpired => {
                    Err(HttpError::UnprocessableEntity(UnprocessableEntity::new(
                        Box::new(e),
                    )))
                    //todo: change message to
                    // build_unprocessable_entity_response("Token is expired".to_string())
                }
                RestorePasswordError::TokenNotFoundError => {
                    Err(HttpError::UnprocessableEntity(UnprocessableEntity::new(
                        Box::new(e),
                    )))
                    //todo: change message to
                    // build_unprocessable_entity_response("Wrong restore password token".to_string())
                }
                _ => Err(HttpError::InternalServerError(InternalServerError::new(
                    Box::new(e),
                ))),
            },
        }
    }
}
