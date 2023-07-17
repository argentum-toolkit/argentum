use argentum_rest_infrastructure::data_type::error::{HttpError, InternalServerError};
use argentum_rest_infrastructure::data_type::HttpResponse;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_account_business::use_case::anonymous_registers::AnonymousRegistersUc;
use argentum_user_account_rest::dto::schema::AnonymousRegistrationResult;
use argentum_user_account_rest::server::handler::AnonymousRegistersTrait;
use hyper::StatusCode;
use std::sync::Arc;

pub struct AnonymousRegistersHandler {
    uc: Arc<AnonymousRegistersUc>,
    id_factory: Arc<UniqueIdFactory>,
}

impl AnonymousRegistersHandler {
    pub fn new(uc: Arc<AnonymousRegistersUc>, id_factory: Arc<UniqueIdFactory>) -> Self {
        AnonymousRegistersHandler { uc, id_factory }
    }
}

impl AnonymousRegistersTrait for AnonymousRegistersHandler {
    fn handle(&self) -> Result<HttpResponse, HttpError> {
        let anonymous_id = self.id_factory.create();

        let result = self.uc.execute(&anonymous_id);

        match result {
            Ok((anonymous, session)) => {
                let id = self.id_factory.id_to_uuid(&anonymous.id);
                let schema = AnonymousRegistrationResult::new(id, session.token);

                Ok(HttpResponse::new(StatusCode::CREATED, Box::new(schema)))
            }

            Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
                Box::new(e),
            ))),
        }
    }
}
