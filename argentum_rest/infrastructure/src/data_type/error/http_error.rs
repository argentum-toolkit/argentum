use crate::data_type::SerializableBody;
use crate::data_type::error::method_not_allowed::MethodNotAllowedError;
use crate::data_type::error::{
    BadRequestError, Conflict, Forbidden, InternalServerError, NotFoundError, NotImplementedError,
    ServiceUnavailableError, Unauthorized, UnprocessableEntity,
};
use serde::Serialize;

#[derive(Serialize)]
pub enum HttpError {
    //5xx
    InternalServerError(InternalServerError),

    NotImplemented(NotImplementedError),

    ServiceUnavailable(ServiceUnavailableError),

    //4xx
    BadRequest(BadRequestError),

    Unauthorized(Unauthorized),

    Forbidden(Forbidden),

    NotFound(NotFoundError),

    RouteNotFound(NotFoundError),

    MethodNotAllowed(MethodNotAllowedError),

    Conflict(Conflict),

    UnprocessableEntity(UnprocessableEntity),
}

impl SerializableBody for HttpError {}
