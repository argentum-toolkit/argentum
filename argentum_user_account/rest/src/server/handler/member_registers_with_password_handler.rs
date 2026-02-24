use crate::dto::operation_response_enum::MemberRegistersWithPasswordOperationResponseEnum;
use crate::dto::request::MemberRegistersWithPasswordRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::entity::user::User;

use async_trait::async_trait;

#[async_trait]
pub trait MemberRegistersWithPasswordTrait: Send + Sync {
    type Id: Id;

    async fn handle(
        &self,
        _req: MemberRegistersWithPasswordRequest,
        _user: User<Self::Id>,
    ) -> Result<MemberRegistersWithPasswordOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
