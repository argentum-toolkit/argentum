use actix_web::{web, Scope};

use crate::actix_web::handler::anonymous_registers;
use crate::actix_web::handler::login_with_password;
use crate::actix_web::handler::register_with_password;
use crate::actix_web::handler::restore_password::{
    change_password_with_token, request_restore_token,
};

pub fn create_user_account_scope() -> Scope {
    web::scope("/api/v1/user")
        .route("/anonymous-register", web::post().to(anonymous_registers))
        .route("/register", web::post().to(register_with_password))
        .route("/password-login", web::post().to(login_with_password))
        .route(
            "/restore-password/token-request",
            web::post().to(request_restore_token),
        )
        .route(
            "/restore-password/change-password",
            web::post().to(change_password_with_token),
        )
}
