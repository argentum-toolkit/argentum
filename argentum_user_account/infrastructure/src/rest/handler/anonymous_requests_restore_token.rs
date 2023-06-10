use crate::api::dto::request::AnonymousRequestsRestoreTokenRequest;
use crate::api::server::handler::AnonymousRequestsRestoreTokenTrait;
use crate::rest::transformer::DtoToAnonymousRequestsRestoreTokenParams;
use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::data_type::error::{Conflict, HttpError, InternalServerError};
use argentum_rest_infrastructure::data_type::HttpResponse;
use argentum_user_account_api::models::EmptyResponse;
use argentum_user_account_business::use_case::restore_password::anonymous_requests_restore_token::AnonymousRequestsRestoreTokenUc;
use argentum_user_account_business::use_case::restore_password::error::RestorePasswordError;
use argentum_user_business::entity::user::User;
use hyper::StatusCode;
use std::sync::Arc;

pub struct AnonymousRequestsRestoreTokenHandler {
    uc: Arc<AnonymousRequestsRestoreTokenUc>,
    logger: Arc<dyn LoggerTrait>,
    dto_to_anonymous_requests_restore_token_params: Arc<DtoToAnonymousRequestsRestoreTokenParams>,
}

impl AnonymousRequestsRestoreTokenHandler {
    pub fn new(
        uc: Arc<AnonymousRequestsRestoreTokenUc>,
        logger: Arc<dyn LoggerTrait>,
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

impl AnonymousRequestsRestoreTokenTrait for AnonymousRequestsRestoreTokenHandler {
    fn handle(
        &self,
        req: AnonymousRequestsRestoreTokenRequest,
        _user: User,
    ) -> Result<HttpResponse, HttpError> {
        let email = self
            .dto_to_anonymous_requests_restore_token_params
            .transform(req)?;

        let result = self.uc.execute(email);

        match result {
            Ok(_) => {
                let dto = EmptyResponse::from(serde_json::Value::Null);

                Ok(HttpResponse::new(StatusCode::OK, Box::new(dto)))
            }

            Err(e) => match e {
                RestorePasswordError::UserNotFoundError => {
                    self.logger.warning(format!("{:?}", e));

                    Err(HttpError::Conflict(Conflict::new(Box::new(e))))
                }
                _ => Err(HttpError::InternalServerError(InternalServerError::new(
                    Box::new(e),
                ))),
            },
        }
    }
}
