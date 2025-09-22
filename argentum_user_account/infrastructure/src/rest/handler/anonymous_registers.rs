use argentum_rest_infrastructure::data_type::error::{HttpError, InternalServerError};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::use_case::anonymous_registers::AnonymousRegistersUc;
use argentum_user_account_rest::dto::operation_response_enum::AnonymousRegistersOperationResponseEnum;
use argentum_user_account_rest::dto::request::AnonymousRegistersRequest;
use argentum_user_account_rest::dto::response::AnonymousRegisteredSuccessfullyResponse;
use argentum_user_account_rest::dto::schema::AnonymousRegistrationResult;
use argentum_user_account_rest::server::handler::AnonymousRegistersTrait;
use std::sync::Arc;

use async_trait::async_trait;

pub struct AnonymousRegistersHandler {
    uc: Arc<AnonymousRegistersUc<UniqueId>>,
}

impl AnonymousRegistersHandler {
    pub fn new(uc: Arc<AnonymousRegistersUc<UniqueId>>) -> Self {
        AnonymousRegistersHandler { uc }
    }
}

#[async_trait]
impl AnonymousRegistersTrait for AnonymousRegistersHandler {
    type Id = UniqueId;

    async fn handle(
        &self,
        _req: AnonymousRegistersRequest,
    ) -> Result<AnonymousRegistersOperationResponseEnum, HttpError> {
        let anonymous_id = UniqueId::new();

        let result = self.uc.execute(anonymous_id).await;

        match result {
            Ok((anonymous, session)) => {
                let schema = AnonymousRegistrationResult::new(anonymous.id.into(), &session.token);

                Ok(AnonymousRegistersOperationResponseEnum::Status201(
                    AnonymousRegisteredSuccessfullyResponse::new_application_json(schema.clone()),
                ))
            }

            Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
                Arc::new(e),
            ))),
        }
    }
}
