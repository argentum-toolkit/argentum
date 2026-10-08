use crate::rest::transformer::DtoToGuestWithTokenChangesPasswordParams;
use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::data_type::error::{
    BadRequestError, HttpError, InternalServerError, UnprocessableEntity,
};
use argentum_standard_business::data_type::id::Id;
use argentum_standard_business::invariant_violation::Violations;
use argentum_user_account_business::use_case::restore_password::error::RestorePasswordError;
use argentum_user_account_business::use_case::restore_password::guest_with_token_changes_password::GuestWithTokenChangesPasswordUc;
use argentum_user_account_rest::dto::operation_response_enum::GuestWithTokenChangesPasswordOperationResponseEnum;
use argentum_user_account_rest::dto::request::GuestWithTokenChangesPasswordRequest;
use argentum_user_account_rest::dto::response::EmptyOkResponse;
use argentum_user_account_rest::dto::schema::EmptyResponse;
use argentum_user_account_rest::server::handler::GuestWithTokenChangesPasswordTrait;
use argentum_user_business::entity::user::User;
use std::sync::Arc;

use async_trait::async_trait;

pub struct GuestWithTokenChangesPasswordHandler<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    uc: Arc<GuestWithTokenChangesPasswordUc<I, L>>,
    dto_to_guest_with_token_changes_password_params: Arc<DtoToGuestWithTokenChangesPasswordParams>,
}

impl<I, L> GuestWithTokenChangesPasswordHandler<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        uc: Arc<GuestWithTokenChangesPasswordUc<I, L>>,
        dto_to_guest_with_token_changes_password_params: Arc<
            DtoToGuestWithTokenChangesPasswordParams,
        >,
    ) -> Self {
        GuestWithTokenChangesPasswordHandler {
            uc,
            dto_to_guest_with_token_changes_password_params,
        }
    }
}

#[async_trait]
impl<I, L> GuestWithTokenChangesPasswordTrait for GuestWithTokenChangesPasswordHandler<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    type Id = I;

    async fn handle(
        &self,
        req: GuestWithTokenChangesPasswordRequest,
        _user: User<Self::Id>,
    ) -> Result<GuestWithTokenChangesPasswordOperationResponseEnum, HttpError> {
        let (token, pw) = self
            .dto_to_guest_with_token_changes_password_params
            .transform(req)?;

        let result = self.uc.execute(&token, &pw).await;

        match result {
            Ok(_) => Ok(
                GuestWithTokenChangesPasswordOperationResponseEnum::Status200(
                    EmptyOkResponse::new_application_json(EmptyResponse::new()),
                ),
            ),
            Err(e) => match e {
                RestorePasswordError::TokenExpired | RestorePasswordError::TokenNotFoundError => {
                    Err(HttpError::UnprocessableEntity(UnprocessableEntity::new(
                        Box::new(e),
                    )))
                }
                RestorePasswordError::PasswordSpecificationError(_) => {
                    Err(HttpError::BadRequest(BadRequestError::new(
                        Violations::new(vec![e.to_string()], None),
                        Violations::new(vec![], None),
                        Violations::new(vec![], None),
                        Violations::new(vec![], None),
                    )))
                }
                _ => Err(HttpError::InternalServerError(InternalServerError::new(
                    Arc::new(e),
                ))),
            },
        }
    }
}
