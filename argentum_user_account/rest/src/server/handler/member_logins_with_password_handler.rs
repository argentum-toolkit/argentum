use crate::dto::operation_response_enum::MemberLoginsWithPasswordOperationResponseEnum;
use crate::dto::request::MemberLoginsWithPasswordRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::entity::user::User;

use async_trait::async_trait;

#[async_trait]
pub trait MemberLoginsWithPasswordTrait: Send + Sync {
    type Id: Id;

    async fn handle(
        &self,
        _req: MemberLoginsWithPasswordRequest,
        _user: User<Self::Id>,
    ) -> Result<MemberLoginsWithPasswordOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
