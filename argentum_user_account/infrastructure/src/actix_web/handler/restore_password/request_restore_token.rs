use actix_web::{web, HttpResponse, Responder};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_infrastructure::actix_web::http_problem::build_internal_server_error_response;
use argentum_standard_infrastructure::error::InternalError;
use argentum_user_account_api::models::RequestRestoreTokenSchema;
use argentum_user_account_business::use_case::restore_password::anonymous_requests_restore_token::AnonymousRequestsRestoreToken;
use argentum_user_account_business::use_case::restore_password::error::RestorePasswordError;
use std::sync::Arc;

#[derive(serde::Serialize)]
struct Empty {}

pub async fn request_restore_token(
    uc: web::Data<Arc<AnonymousRequestsRestoreToken>>,
    logger: web::Data<Arc<dyn LoggerTrait>>,
    json: web::Json<RequestRestoreTokenSchema>,
) -> impl Responder {
    let email = match EmailAddress::new(json.email.clone()) {
        Ok(email) => email,
        Err(_) => return HttpResponse::BadRequest().body("Wrong email address"),
    };

    let result = uc.execute(email);

    match result {
        Ok(_) => HttpResponse::Ok().json(Empty {}),

        Err(e) => match e {
            RestorePasswordError::UserNotFoundError => {
                logger.info(format!("{:?}", e));

                //Return Ok to hide sensitive data (e.g: email exists in DB)
                HttpResponse::Ok().json(Empty {})
            }
            _ => {
                logger.error(format!("{:?}", InternalError::Server(Box::new(e))));

                build_internal_server_error_response()
            }
        },
    }
}
