use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::data_type::error::{
    HttpError, InternalServerError, Unauthorized,
};
use argentum_standard_business::data_type::id::Id;
use argentum_user_account_business::use_case::member_deletes_account::{
    MemberDeletesAccountError, MemberDeletesAccountUc,
};
use argentum_user_account_rest::dto::operation_response_enum::MemberDeletesAccountOperationResponseEnum;
use argentum_user_account_rest::dto::request::MemberDeletesAccountRequest;
use argentum_user_account_rest::dto::response::Status204Response;
use argentum_user_account_rest::server::handler::MemberDeletesAccountTrait;
use argentum_user_business::entity::user::{User, UserTrait};
use async_trait::async_trait;
use std::sync::Arc;

pub struct MemberDeletesAccountHandler<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    uc: Arc<MemberDeletesAccountUc<I, L>>,
}

impl<I, L> MemberDeletesAccountHandler<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(uc: Arc<MemberDeletesAccountUc<I, L>>) -> Self {
        Self { uc }
    }
}

#[async_trait]
impl<I, L> MemberDeletesAccountTrait for MemberDeletesAccountHandler<I, L>
where
    I: Id + 'static,
    L: LoggerTrait,
{
    type Id = I;

    async fn handle(
        &self,
        req: MemberDeletesAccountRequest,
        user: User<Self::Id>,
    ) -> Result<MemberDeletesAccountOperationResponseEnum, HttpError> {
        let member = match user {
            User::Member(m) => m,
            User::Guest(_) => {
                return Err(HttpError::Unauthorized(Unauthorized::new(
                    "Endpoint is available only for members",
                )));
            }
        };

        let result = self.uc.execute(member.id(), req.body.password.into()).await;

        match result {
            Ok(_) => Ok(MemberDeletesAccountOperationResponseEnum::Status204(
                Status204Response::new_no_content(),
            )),
            Err(MemberDeletesAccountError::WrongPassword) => Err(HttpError::Unauthorized(
                Unauthorized::new(&format!("{}", MemberDeletesAccountError::WrongPassword)),
            )),
            Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
                Arc::new(e),
            ))),
        }
    }
}
