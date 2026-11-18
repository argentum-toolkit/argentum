use crate::dto::operation_response_enum::MemberDeletesAccountOperationResponseEnum;
use crate::dto::request::MemberDeletesAccountRequest;
use argentum_rest_infrastructure::data_type::error::{HttpError, NotImplementedError};
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::entity::user::User;

use async_trait::async_trait;

#[async_trait]
pub trait MemberDeletesAccountTrait: Send + Sync {
    type Id: Id;

    async fn handle(
        &self,
        _req: MemberDeletesAccountRequest,
        _user: User<Self::Id>,
    ) -> Result<MemberDeletesAccountOperationResponseEnum, HttpError> {
        Err(HttpError::NotImplemented(NotImplementedError::new()))
    }
}
