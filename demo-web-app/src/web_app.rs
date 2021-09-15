use crate::di::DiC;
use actix_web::{web, App, HttpServer};

use actix_web::middleware::Logger;
use argentum_log_business::LoggerTrait;
use argentum_standard_infrastructure::actix_web::http_problem::json_error_handler;
use argentum_standard_infrastructure::actix_web::route::create_default_service;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_account_infrastructure::actix_web::middleware::authentication::AuthenticationMiddlewareFactory;
use argentum_user_account_infrastructure::actix_web::route::create_user_account_scope;
use std::sync::Arc;

pub async fn start_server(di: DiC) -> std::io::Result<()> {
    let id_factory: Arc<UniqueIdFactory> = di.id_factory.clone();
    let logger: Arc<dyn LoggerTrait> = di.logger.clone();
    let user_authenticates_with_token_uc = di.user_authenticates_with_token.clone();
    let anonymous_registers_uc = di.anonymous_registers_uc.clone();
    let user_registers_with_password_uc = di.user_registers_with_password_uc.clone();
    let user_logins_with_password_uc = di.user_logins_with_password_uc.clone();
    let anonymous_requests_restore_token_uc = di.anonymous_requests_restore_token_uc.clone();
    let anonymous_with_token_changes_password_uc =
        di.anonymous_with_token_changes_password_uc.clone();
    let data = web::Data::new(di);
    {
        HttpServer::new(move || {
            let json_config = web::JsonConfig::default().error_handler(json_error_handler);

            App::new()
                .app_data(json_config)
                .app_data(web::Data::new(anonymous_registers_uc.clone()))
                .app_data(web::Data::new(user_logins_with_password_uc.clone()))
                .app_data(web::Data::new(user_registers_with_password_uc.clone()))
                .app_data(web::Data::new(id_factory.clone()))
                .app_data(web::Data::new(logger.clone()))
                .app_data(web::Data::new(user_authenticates_with_token_uc.clone()))
                .app_data(web::Data::new(anonymous_requests_restore_token_uc.clone()))
                .app_data(web::Data::new(
                    anonymous_with_token_changes_password_uc.clone(),
                ))
                .app_data(data.clone())
                .wrap(AuthenticationMiddlewareFactory)
                .wrap(Logger::default())
                .service(create_user_account_scope())
                .default_service(create_default_service())
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
}
