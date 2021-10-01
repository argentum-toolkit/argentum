use actix_web::{web, HttpResponse, Responder};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_infrastructure::actix_web::http_problem::{
    build_internal_server_error_response, build_unprocessable_entity_response,
};
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::error::InternalError;
use argentum_user_account_api::models::{
    RegistrationWithPasswordResult, RegistrationWithPasswordSchema,
};
use argentum_user_account_business::use_case::user_registers_with_password::{
    RegistrationError, UserRegistersWithPasswordUc,
};
use argentum_user_business::value_object::name::Name;
use std::sync::Arc;

pub async fn register_with_password(
    uc: web::Data<Arc<UserRegistersWithPasswordUc>>,
    id_factory: web::Data<Arc<UniqueIdFactory>>,
    logger: web::Data<Arc<dyn LoggerTrait>>,
    json: web::Json<RegistrationWithPasswordSchema>,
) -> impl Responder {
    let user_id = id_factory.get_ref().create();

    let json_name = json.name.clone();

    let result = uc.execute(
        user_id,
        Name::new(json_name.first, json_name.last.unwrap()).unwrap(),
        EmailAddress::new(json.email.clone()).unwrap(),
        json.password.clone(),
    );

    match result {
        Ok(user) => {
            let id = id_factory.id_to_uuid(&user.id);

            let schema = RegistrationWithPasswordResult::new(id);
            HttpResponse::Created().json(&schema)
        }

        Err(e) => match e {
            RegistrationError::EmailAlreadyExists => {
                build_unprocessable_entity_response(format!("{}", e))
            }
            _ => {
                logger.error(format!("{:?}", InternalError::Server(Box::new(e))));

                build_internal_server_error_response()
            }
        },
    }
}
