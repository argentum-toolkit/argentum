use argentum_rest_infrastructure::data_type::error::{HttpError, InternalServerError};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::use_case::guest_registers::GuestRegistersUc;
use argentum_user_account_rest::dto::operation_response_enum::GuestRegistersOperationResponseEnum;
use argentum_user_account_rest::dto::request::GuestRegistersRequest;
use argentum_user_account_rest::dto::response::GuestRegisteredSuccessfullyResponse;
use argentum_user_account_rest::dto::schema::GuestRegistrationResult;
use argentum_user_account_rest::server::handler::GuestRegistersTrait;
use std::sync::Arc;

use async_trait::async_trait;

pub struct GuestRegistersHandler {
    uc: Arc<GuestRegistersUc<UniqueId>>,
}

impl GuestRegistersHandler {
    pub fn new(uc: Arc<GuestRegistersUc<UniqueId>>) -> Self {
        GuestRegistersHandler { uc }
    }
}

#[async_trait]
impl GuestRegistersTrait for GuestRegistersHandler {
    type Id = UniqueId;

    async fn handle(
        &self,
        _req: GuestRegistersRequest,
    ) -> Result<GuestRegistersOperationResponseEnum, HttpError> {
        let guest_id = UniqueId::new();

        let result = self.uc.execute(guest_id).await;

        match result {
            Ok((guest, session)) => {
                let schema = GuestRegistrationResult::new(guest.id.into(), session.token.clone());

                Ok(GuestRegistersOperationResponseEnum::Status201(
                    GuestRegisteredSuccessfullyResponse::new_application_json(schema.clone()),
                ))
            }

            Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
                Arc::new(e),
            ))),
        }
    }
}
