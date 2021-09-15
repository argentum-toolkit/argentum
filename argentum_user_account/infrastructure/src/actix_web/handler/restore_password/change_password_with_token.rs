use actix_web::{web, HttpResponse, Responder};
use argentum_user_account_api::models::ChangePasswordSchema;
use argentum_user_account_business::use_case::restore_password::error::RestorePasswordError;
use argentum_user_account_business::use_case::restore_password::anonymous_with_token_changes_password::AnonymousWithTokenChangesPassword;
use std::sync::Arc;
use argentum_standard_infrastructure::actix_web::http_problem::{build_internal_server_error_response, build_unprocessable_entity_response};
use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::error::InternalError;

#[derive(serde::Serialize)]
struct Empty {}

pub async fn change_password_with_token(
    uc: web::Data<Arc<AnonymousWithTokenChangesPassword>>,
    logger: web::Data<Arc<dyn LoggerTrait>>,
    json: web::Json<ChangePasswordSchema>,
) -> impl Responder {
    let result = uc.execute(json.token.clone(), json.password.clone());

    match result {
        Ok(_) => HttpResponse::Ok().json(Empty {}),

        Err(e) => match e {
            RestorePasswordError::TokenExpired => {
                build_unprocessable_entity_response("Token is expired".to_string())
            }
            RestorePasswordError::TokenNotFoundError => {
                build_unprocessable_entity_response("Wrong restore password token".to_string())
            }
            _ => {
                logger.error(format!("{:?}", InternalError::Server(Box::new(e))));

                build_internal_server_error_response()
            }
        },
    }
}
