use actix_web::{web, HttpResponse, Responder};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_infrastructure::actix_web::http_problem::build_internal_server_error_response;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::error::InternalError;
use argentum_user_account_api::models::AnonymousRegistrationResult;
use argentum_user_account_business::use_case::anonymous_registers::{
    AnonymousRegistersUc, AnonymousRegistrationError,
};
use std::sync::Arc;

pub async fn anonymous_registers(
    uc: web::Data<Arc<AnonymousRegistersUc>>,
    id_factory: web::Data<Arc<UniqueIdFactory>>,
    logger: web::Data<Arc<dyn LoggerTrait>>,
) -> impl Responder {
    let anonymous_id = id_factory.get_ref().create();

    let result = uc.execute(&anonymous_id);

    match result {
        Ok((anonymous, session)) => {
            let id = id_factory.id_to_uuid(anonymous.id);

            let schema = AnonymousRegistrationResult::new(id, session.token);
            HttpResponse::Created().json(&schema)
        }

        Err(e) => match e {
            AnonymousRegistrationError::SavingAnonymousError(_)
            | AnonymousRegistrationError::SavingSessionError(_) => {
                logger.error(format!("{:?}", InternalError::Server(Box::new(e))));

                build_internal_server_error_response()
            }
        },
    }
}
