use argentum_user_account_rest::dto::request::{
    AnonymousWithTokenChangesPasswordRequest,
};
use argentum_user_account_rest::server::handler::{
    AnonymousWithTokenChangesPasswordTrait,
};
use crate::rest::transformer::{ DtoToAnonymousWithTokenChangesPasswordParams};
use argentum_rest_infrastructure::data_type::error::{ HttpError, InternalServerError, UnprocessableEntity};
use argentum_user_account_rest::dto::schema::EmptyResponse;
use argentum_user_account_business::use_case::restore_password::error::RestorePasswordError;
use argentum_user_business::entity::user::User;
use std::sync::Arc;
use argentum_user_account_business::use_case::restore_password::anonymous_with_token_changes_password::AnonymousWithTokenChangesPasswordUc;
use argentum_user_account_rest::dto::operation_response_enum::AnonymousWithTokenChangesPasswordOperationResponseEnum;
use argentum_user_account_rest::dto::response::EmptyOkResponse;

pub struct AnonymousWithTokenChangesPasswordHandler {
    uc: Arc<AnonymousWithTokenChangesPasswordUc>,
    dto_to_anonymous_with_token_changes_password_params:
        Arc<DtoToAnonymousWithTokenChangesPasswordParams>,
}

impl AnonymousWithTokenChangesPasswordHandler {
    pub fn new(
        uc: Arc<AnonymousWithTokenChangesPasswordUc>,
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
    ) -> Result<AnonymousWithTokenChangesPasswordOperationResponseEnum, HttpError> {
        let (token, pw) = self
            .dto_to_anonymous_with_token_changes_password_params
            .transform(req)?;

        let result = self.uc.execute(token, pw);

        match result {
            Ok(_) => Ok(
                AnonymousWithTokenChangesPasswordOperationResponseEnum::Status200(
                    EmptyOkResponse::new_application_json(EmptyResponse::new()),
                ),
            ),
            Err(e) => match e {
                RestorePasswordError::TokenExpired | RestorePasswordError::TokenNotFoundError => {
                    Err(HttpError::UnprocessableEntity(UnprocessableEntity::new(
                        Box::new(e),
                    )))
                }
                _ => Err(HttpError::InternalServerError(InternalServerError::new(
                    Box::new(e),
                ))),
            },
        }
    }
}
