use crate::rest::transformer::DtoToGuestRequestsRestoreTokenParams;
use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::data_type::error::{Conflict, HttpError, InternalServerError};
use argentum_standard_business::data_type::id::Id;
use argentum_user_account_business::use_case::restore_password::error::RestorePasswordError;
use argentum_user_account_business::use_case::restore_password::guest_requests_restore_token::GuestRequestsRestoreTokenUc;
use argentum_user_account_rest::dto::operation_response_enum::GuestRequestsRestoreTokenOperationResponseEnum;
use argentum_user_account_rest::dto::request::GuestRequestsRestoreTokenRequest;
use argentum_user_account_rest::dto::response::EmptyOkResponse;
use argentum_user_account_rest::dto::schema::EmptyResponse;
use argentum_user_account_rest::server::handler::GuestRequestsRestoreTokenTrait;
use argentum_user_business::entity::user::User;
use async_trait::async_trait;
use std::sync::Arc;

pub struct GuestRequestsRestoreTokenHandler<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    uc: Arc<GuestRequestsRestoreTokenUc<I, L>>,
    logger: Arc<L>,
    dto_to_guest_requests_restore_token_params: Arc<DtoToGuestRequestsRestoreTokenParams>,
}

impl<I, L> GuestRequestsRestoreTokenHandler<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        uc: Arc<GuestRequestsRestoreTokenUc<I, L>>,
        logger: Arc<L>,
        dto_to_guest_requests_restore_token_params: Arc<DtoToGuestRequestsRestoreTokenParams>,
    ) -> Self {
        GuestRequestsRestoreTokenHandler {
            uc,
            logger,
            dto_to_guest_requests_restore_token_params,
        }
    }
}

#[async_trait]
impl<I, L> GuestRequestsRestoreTokenTrait for GuestRequestsRestoreTokenHandler<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    type Id = I;

    async fn handle(
        &self,
        req: GuestRequestsRestoreTokenRequest,
        _user: User<I>,
    ) -> Result<GuestRequestsRestoreTokenOperationResponseEnum, HttpError> {
        let email = self
            .dto_to_guest_requests_restore_token_params
            .transform(req)?;

        let result = self.uc.execute(email).await;

        match result {
            Ok(_) => Ok(GuestRequestsRestoreTokenOperationResponseEnum::Status200(
                EmptyOkResponse::new_application_json(EmptyResponse::new()),
            )),

            Err(e) => match e {
                RestorePasswordError::UserNotFoundError => {
                    self.logger.warning(format!("{e:?}"));

                    Err(HttpError::Conflict(Conflict::new(Box::new(e))))
                }
                _ => Err(HttpError::InternalServerError(InternalServerError::new(
                    Arc::new(e),
                ))),
            },
        }
    }
}
