use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::data_type::error::{BadRequestError, HttpError};
use argentum_standard_business::invariant_violation::Violations;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::use_case::member_confirms_email::MemberConfirmsEmailUcTrait;
use argentum_user_account_rest::{
    dto::{
        operation_response_enum::MemberVerifiesEmailOperationResponseEnum,
        request::MemberVerifiesEmailRequest, response::EmptyOkResponse, schema::EmptyResponse,
    },
    server::handler::MemberVerifiesEmailTrait,
};
use argentum_user_business::entity::user::User;
use async_trait::async_trait;

pub struct MemberVerifiesEmailHandler<L>
where
    L: LoggerTrait,
{
    uc: Arc<dyn MemberConfirmsEmailUcTrait<Id = UniqueId>>,
    logger: Arc<L>,
}

impl<L> MemberVerifiesEmailHandler<L>
where
    L: LoggerTrait,
{
    pub fn new(uc: Arc<dyn MemberConfirmsEmailUcTrait<Id = UniqueId>>, logger: Arc<L>) -> Self {
        Self { uc, logger }
    }
}

#[async_trait]
impl<L> MemberVerifiesEmailTrait for MemberVerifiesEmailHandler<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn handle(
        &self,
        req: MemberVerifiesEmailRequest, //member's key
        _user: User<Self::Id>,           //guest's bearer token
    ) -> Result<MemberVerifiesEmailOperationResponseEnum, HttpError> {
        match self.uc.execute(&req.body.key).await {
            Ok(_) => Ok(MemberVerifiesEmailOperationResponseEnum::Status200(
                EmptyOkResponse::new_application_json(EmptyResponse::new()),
            )),
            Err(e) => {
                self.logger.warning(format!("Can't confirm email: {}", e));

                Err(HttpError::BadRequest(BadRequestError::new(
                    Violations::new(vec![e.to_string()], None),
                    Violations::new(vec![], None),
                    Violations::new(vec![], None),
                    Violations::new(vec![], None),
                )))
            }
        }
    }
}
