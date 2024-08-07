use crate::data_type::error::method_not_allowed::MethodNotAllowedError;
use crate::data_type::error::{
    BadRequestError, Conflict, InternalServerError, NotFoundError, NotImplementedError,
    Unauthorized, UnprocessableEntity,
};
use crate::data_type::SerializableBody;
use serde::Serialize;

#[derive(Serialize)]
pub enum HttpError {
    //5xx
    InternalServerError(InternalServerError),

    NotImplemented(NotImplementedError),

    //4xx
    BadRequest(BadRequestError),

    Unauthorized(Unauthorized),

    NotFound(NotFoundError),

    RouteNotFound(NotFoundError),

    MethodNotAllowed(MethodNotAllowedError),

    Conflict(Conflict),

    UnprocessableEntity(UnprocessableEntity),
}

impl SerializableBody for HttpError {}
