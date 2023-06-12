use crate::data_type::error::{HttpError, InternalServerError, Unauthorized};
use crate::data_type::AuthHeaderParams;
use argentum_user_account_business::use_case::user_authenticates_with_token::AuthenticationError as BusinessAuthenticationError;
use argentum_user_account_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use argentum_user_business::entity::user::User;
use std::sync::Arc;

pub struct BearerAuthenticator {
    uc: Arc<UserAuthenticatesWithTokenUc>,
}

impl BearerAuthenticator {
    pub fn new(uc: Arc<UserAuthenticatesWithTokenUc>) -> Self {
        Self { uc }
    }

    pub fn auth(&self, headers: &AuthHeaderParams) -> Result<User, HttpError> {
        let header = headers.authorization.clone();

        match header.find("Bearer ") {
            Some(0) => (),
            _ => {
                return Err(HttpError::Unauthorized(Unauthorized::new(
                    "The expected format of `Authorization` header is `Bearer {token}`".to_string(),
                )));
            }
        }

        let token: String = header.chars().skip(7).collect();

        match self.uc.execute(token) {
            Ok(user) => Ok(user),
            Err(BusinessAuthenticationError::UserNotFound)
            | Err(BusinessAuthenticationError::WrongToken) => Err(HttpError::Unauthorized(
                Unauthorized::new("can't authenticate".to_string()),
            )),
            Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
                Box::new(e),
            ))),
        }
    }
}
