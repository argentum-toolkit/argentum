use phf::phf_map;

#[derive(Clone)]
pub struct StatusCode {
    pub code: u16,
    pub reason: &'static str,
}

impl StatusCode {
    const fn c(code: u16, reason: &'static str) -> StatusCode {
        StatusCode { code, reason }
    }
}

//2xx
pub const OK: StatusCode = StatusCode::c(200, "Ok");
pub const CREATED: StatusCode = StatusCode::c(201, "Created");

//4xx
pub const BAD_REQUEST: StatusCode = StatusCode::c(400, "Bad Request");
pub const UNAUTHORIZED: StatusCode = StatusCode::c(401, "Unauthorized");
pub const FORBIDDEN: StatusCode = StatusCode::c(403, "Forbidden");
pub const NOT_FOUND: StatusCode = StatusCode::c(404, "Not Found");
pub const METHOD_NOT_ALLOWED: StatusCode = StatusCode::c(405, "Method Not Allowed");
pub const CONFLICT: StatusCode = StatusCode::c(409, "Conflict");
pub const IM_A_TEAPOT: StatusCode = StatusCode::c(418, "I'm a teapot");
pub const UNPROCESSABLE_CONTENT: StatusCode = StatusCode::c(422, "Unprocessable Content");

//5xx
pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode::c(500, "Internal Server Error");
pub const NOT_IMPLEMENTED: StatusCode = StatusCode::c(501, "Not Implemented");

static STATUS_CODES: phf::Map<u16, StatusCode> = phf_map! {
    200u16 => OK,
    400u16 => BAD_REQUEST,
    401u16 => UNAUTHORIZED,
    403u16 => FORBIDDEN,
    404u16 => NOT_FOUND,
    409u16 => CONFLICT,
    418u16 => IM_A_TEAPOT,
    500u16 => INTERNAL_SERVER_ERROR,
};
