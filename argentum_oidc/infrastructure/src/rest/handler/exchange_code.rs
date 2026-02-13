use argentum_oidc_business::use_case::{
    UserExchangesCodeToTokenUc, error::ExchangesCodeToTokenError,
};
use argentum_oidc_rest::{
    dto::{
        operation_response_enum::ExchangeCodeOperationResponseEnum, request::ExchangeCodeRequest,
        response::CodeExchangedSuccessfullyResponse, schema::ExchangeCodeResult,
    },
    server::handler::ExchangeCodeTrait,
};
use argentum_rest_infrastructure::data_type::error::{HttpError, InternalServerError};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ExchangeCodeHandler {
    uc: Arc<UserExchangesCodeToTokenUc<UniqueId>>,
}

impl ExchangeCodeHandler {
    pub fn new(uc: Arc<UserExchangesCodeToTokenUc<UniqueId>>) -> Self {
        Self { uc }
    }
}

#[async_trait]
impl ExchangeCodeTrait for ExchangeCodeHandler {
    type Id = UniqueId;

    async fn handle(
        &self,
        req: ExchangeCodeRequest,
    ) -> Result<ExchangeCodeOperationResponseEnum, HttpError> {
        match self.uc.execute(&req.body.code).await {
            Ok(data) => Ok(ExchangeCodeOperationResponseEnum::Status201(
                CodeExchangedSuccessfullyResponse::new_application_json(ExchangeCodeResult::new(
                    data.access_token,
                    data.user_id.into(),
                )),
            )),
            Err(ExchangesCodeToTokenError::Exchanger(e)) => Err(HttpError::InternalServerError(
                InternalServerError::new(Arc::new(e)),
            )),
        }
    }
}
