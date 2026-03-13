use crate::rest::transformer::DtoToMemberLoginsWithPasswordParams;
use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::data_type::error::{
    HttpError, InternalServerError, Unauthorized,
};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::use_case::member_logins_with_password::{
    LoginError, MemberLoginsWithPasswordUc,
};
use argentum_user_account_rest::dto::operation_response_enum::MemberLoginsWithPasswordOperationResponseEnum;
use argentum_user_account_rest::dto::request::MemberLoginsWithPasswordRequest;
use argentum_user_account_rest::dto::response::MemberLoggedInSuccessfullyResponse;
use argentum_user_account_rest::dto::schema::LoginResult;
use argentum_user_account_rest::server::handler::MemberLoginsWithPasswordTrait;
use argentum_user_business::entity::user::User;
use async_trait::async_trait;
use std::sync::Arc;

pub struct MemberLoginsWithPasswordHandler<L>
where
    L: LoggerTrait,
{
    uc: Arc<MemberLoginsWithPasswordUc<UniqueId, L>>,
    dto_to_user_logins_with_password_params: Arc<DtoToMemberLoginsWithPasswordParams>,
}

impl<L> MemberLoginsWithPasswordHandler<L>
where
    L: LoggerTrait,
{
    pub fn new(
        uc: Arc<MemberLoginsWithPasswordUc<UniqueId, L>>,
        dto_to_user_logins_with_password_params: Arc<DtoToMemberLoginsWithPasswordParams>,
    ) -> Self {
        Self {
            uc,
            dto_to_user_logins_with_password_params,
        }
    }
}

#[async_trait]
impl<L> MemberLoginsWithPasswordTrait for MemberLoginsWithPasswordHandler<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn handle(
        &self,
        req: MemberLoginsWithPasswordRequest,
        user: User<Self::Id>,
    ) -> Result<MemberLoginsWithPasswordOperationResponseEnum, HttpError> {
        let guest = match user {
            User::Guest(a) => a,
            User::Member(_) => {
                return Err(HttpError::Unauthorized(Unauthorized::new(
                    "Endpoint is available only for guests",
                )));
            }
        };

        let (email, password) = self
            .dto_to_user_logins_with_password_params
            .transform(req)?;

        let result = self.uc.execute(Some(guest), email, password.into()).await;

        match result {
            Ok(session) => {
                let dto = LoginResult::new(session.user_id.into(), session.token);

                Ok(MemberLoginsWithPasswordOperationResponseEnum::Status200(
                    MemberLoggedInSuccessfullyResponse::new_application_json(dto),
                ))
            }
            Err(LoginError::WrongEmailOrPassword) => Err(HttpError::Unauthorized(
                Unauthorized::new(&format!("{}", LoginError::WrongEmailOrPassword)),
            )),
            Err(LoginError::EmailNotConfirmed) => Err(HttpError::Unauthorized(Unauthorized::new(
                &format!("{}", LoginError::EmailNotConfirmed),
            ))),
            Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
                Arc::new(e),
            ))),
        }
    }
}
