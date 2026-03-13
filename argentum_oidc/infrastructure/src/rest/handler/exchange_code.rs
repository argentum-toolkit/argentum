use argentum_log_business::LoggerTrait;
use argentum_oidc_business::{
    service::code_exchanger::CodeExchangerError, use_case::UserExchangesCodeToTokenUc,
};
use argentum_oidc_rest::{
    dto::{
        operation_response_enum::ExchangeCodeOperationResponseEnum, request::ExchangeCodeRequest,
        response::CodeExchangedSuccessfullyResponse, schema::ExchangeCodeResult,
    },
    server::handler::ExchangeCodeTrait,
};
use argentum_rest_infrastructure::data_type::error::{
    HttpError, InternalServerError, Unauthorized,
};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::entity::user::User;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ExchangeCodeHandler<L>
where
    L: LoggerTrait,
{
    uc: Arc<UserExchangesCodeToTokenUc<UniqueId, L>>,
}

impl<L> ExchangeCodeHandler<L>
where
    L: LoggerTrait,
{
    pub fn new(uc: Arc<UserExchangesCodeToTokenUc<UniqueId, L>>) -> Self {
        Self { uc }
    }
}

#[async_trait]
impl<L> ExchangeCodeTrait for ExchangeCodeHandler<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn handle(
        &self,
        req: ExchangeCodeRequest,
        user: User<Self::Id>,
    ) -> Result<ExchangeCodeOperationResponseEnum, HttpError> {
        let guest = match user {
            User::Guest(a) => a,
            User::Member(_) => {
                return Err(HttpError::Unauthorized(Unauthorized::new(
                    "Endpoint is available only for guests",
                )));
            }
        };

        match self
            .uc
            .execute(guest, &req.body.code, &req.body.provider)
            .await
        {
            Ok(data) => Ok(ExchangeCodeOperationResponseEnum::Status201(
                CodeExchangedSuccessfullyResponse::new_application_json(ExchangeCodeResult::new(
                    data.access_token,
                    data.user_id.into(),
                )),
            )),
            Err(e) => match e {
                CodeExchangerError::TokenData(s) => {
                    Err(HttpError::Unauthorized(Unauthorized::new(&s)))
                }
                _ => Err(HttpError::InternalServerError(InternalServerError::new(
                    Arc::new(e),
                ))),
            },
        }
    }
}
