use crate::data_type::AuthHeaderParams;
use crate::data_type::error::{HttpError, InternalServerError, Unauthorized};
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::entity::user::User;
use argentum_user_business::use_case::member_authenticates_with_token::AuthenticationError;
use argentum_user_business::use_case::member_authenticates_with_token::MemberAuthenticatesWithTokenUc;
use std::sync::Arc;

pub struct BearerAuthenticator<I>
where
    I: Id,
{
    uc: Arc<MemberAuthenticatesWithTokenUc<I>>,
}

impl<I> BearerAuthenticator<I>
where
    I: Id,
{
    pub fn new(uc: Arc<MemberAuthenticatesWithTokenUc<I>>) -> Self {
        Self { uc }
    }

    pub async fn auth(&self, headers: &AuthHeaderParams) -> Result<User<I>, HttpError> {
        let header = headers.authorization.clone();

        match header.find("Bearer ") {
            Some(0) => (),
            _ => {
                return Err(HttpError::Unauthorized(Unauthorized::new(
                    "The expected format of `Authorization` header is `Bearer {token}`",
                )));
            }
        }

        let token: String = header.chars().skip(7).collect();

        match self.uc.execute(&token).await {
            Ok(user) => Ok(user),
            Err(AuthenticationError::UserNotFound) | Err(AuthenticationError::WrongToken) => Err(
                HttpError::Unauthorized(Unauthorized::new("can't authenticate")),
            ),
            Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
                Arc::new(e),
            ))),
        }
    }
}
