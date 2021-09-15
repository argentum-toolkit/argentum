use actix_web::{web, HttpResponse, Resource};

use crate::actix_web::http_problem::build_not_found_response;

/// 404 handler
async fn handle_not_found() -> HttpResponse {
    build_not_found_response("Route is not found".to_string())
}

pub fn create_default_service() -> Resource {
    // 404 for request with all methods
    web::resource("").route(web::to(handle_not_found))
}
