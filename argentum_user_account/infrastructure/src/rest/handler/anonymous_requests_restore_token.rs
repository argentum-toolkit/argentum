use crate::rest::transformer::DtoToAnonymousRequestsRestoreTokenParams;
use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::data_type::error::{Conflict, HttpError, InternalServerError};
use argentum_standard_business::data_type::id::Id;
use argentum_user_account_business::use_case::restore_password::anonymous_requests_restore_token::AnonymousRequestsRestoreTokenUc;
use argentum_user_account_business::use_case::restore_password::error::RestorePasswordError;
use argentum_user_account_rest::dto::operation_response_enum::AnonymousRequestsRestoreTokenOperationResponseEnum;
use argentum_user_account_rest::dto::request::AnonymousRequestsRestoreTokenRequest;
use argentum_user_account_rest::dto::response::EmptyOkResponse;
use argentum_user_account_rest::dto::schema::EmptyResponse;
use argentum_user_account_rest::server::handler::AnonymousRequestsRestoreTokenTrait;
use argentum_user_business::entity::user::User;
use async_trait::async_trait;
use std::sync::Arc;

pub struct AnonymousRequestsRestoreTokenHandler<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    uc: Arc<AnonymousRequestsRestoreTokenUc<I, L>>,
    logger: Arc<L>,
    dto_to_anonymous_requests_restore_token_params: Arc<DtoToAnonymousRequestsRestoreTokenParams>,
}

impl<I, L> AnonymousRequestsRestoreTokenHandler<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        uc: Arc<AnonymousRequestsRestoreTokenUc<I, L>>,
        logger: Arc<L>,
        dto_to_anonymous_requests_restore_token_params: Arc<
            DtoToAnonymousRequestsRestoreTokenParams,
        >,
    ) -> Self {
        AnonymousRequestsRestoreTokenHandler {
            uc,
            logger,
            dto_to_anonymous_requests_restore_token_params,
        }
    }
}

#[async_trait]
impl<I, L> AnonymousRequestsRestoreTokenTrait for AnonymousRequestsRestoreTokenHandler<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    type Id = I;

    async fn handle(
        &self,
        req: AnonymousRequestsRestoreTokenRequest,
        _user: User<I>,
    ) -> Result<AnonymousRequestsRestoreTokenOperationResponseEnum, HttpError> {
        let email = self
            .dto_to_anonymous_requests_restore_token_params
            .transform(req)?;

        let result = self.uc.execute(email).await;

        match result {
            Ok(_) => Ok(
                AnonymousRequestsRestoreTokenOperationResponseEnum::Status200(
                    EmptyOkResponse::new_application_json(EmptyResponse::new()),
                ),
            ),

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
