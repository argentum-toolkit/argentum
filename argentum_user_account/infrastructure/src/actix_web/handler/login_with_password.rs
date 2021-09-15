use crate::actix_web::extractor::user::UserContainer;
use actix_web::{web, HttpResponse, Responder};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_infrastructure::actix_web::http_problem::{
    build_internal_server_error_response, build_unprocessable_entity_response,
};
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::error::InternalError;
use argentum_user_account_api::models::{LoginResult, LoginWithPasswordSchema};
use argentum_user_account_api::LoginWithPasswordResponse;
use argentum_user_account_business::use_case::user_logins_with_password::{
    LoginError, UserLoginsWithPasswordUc,
};
use argentum_user_business::entity::user::User;
use std::sync::Arc;

pub async fn login_with_password(
    uc: web::Data<Arc<UserLoginsWithPasswordUc>>,
    id_factory: web::Data<Arc<UniqueIdFactory>>,
    logger: web::Data<Arc<dyn LoggerTrait>>,
    user: UserContainer,
    json: web::Json<LoginWithPasswordSchema>, // ) -> Box<dyn Fn(Json<RegistrationWithPasswordSchema>) -> HttpResponse + '_> {
) -> impl Responder {
    let anonymous = match user.0 {
        User::Anonymous(a) => a,
        User::Authenticated(_) => {
            return HttpResponse::UnprocessableEntity()
                .body("Endpoint is available only for anonymous")
        }
    };

    let result = uc.execute(
        Some(anonymous),
        EmailAddress::new(json.email.clone()).unwrap(),
        json.password.clone(),
    );

    match result {
        Ok(session) => {
            let id = id_factory.id_to_uuid(session.user_id);

            let schema = LoginWithPasswordResponse::OK(LoginResult::new(id, session.token));
            HttpResponse::Ok().json(&schema)
        }

        Err(e) => match e {
            LoginError::SaveSession | LoginError::GetUserError(_) => {
                logger.error(format!("{:?}", InternalError::Server(Box::new(e))));

                build_internal_server_error_response()
            }
            LoginError::WrongEmailOrPassword => {
                build_unprocessable_entity_response(format!("{}", e))
            }
        },
    }
}
