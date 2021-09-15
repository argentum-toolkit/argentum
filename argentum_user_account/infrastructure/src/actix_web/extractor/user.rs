use actix_web::dev::Payload;
use actix_web::{error, Error, FromRequest, HttpRequest};
use argentum_user_business::entity::user::User;

use derive_more::{Display, Error};
use futures_util::future::{err, ok, Ready};

#[derive(Clone)]
pub struct UserContainer(pub User);

#[derive(Debug, Display, Error)]
#[display(fmt = "Wrong header: {}", name)]
struct ExtractionError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for ExtractionError {}

impl UserContainer {}

impl FromRequest for UserContainer {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;
    type Config = ();

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions_mut();
        if let Some(user) = extensions.get::<User>() {
            return ok(UserContainer(user.clone()));
        }

        //TODO: log
        err(error::ErrorInternalServerError(
            "Can not get an authenticated user",
        ))
    }
}
