use crate::rest::transformer::DtoToUserLoginsWithPasswordParams;
use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::data_type::error::{
    HttpError, InternalServerError, Unauthorized,
};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::use_case::user_logins_with_password::{
    LoginError, UserLoginsWithPasswordUc,
};
use argentum_user_account_rest::dto::operation_response_enum::UserLoginsWithPasswordOperationResponseEnum;
use argentum_user_account_rest::dto::request::UserLoginsWithPasswordRequest;
use argentum_user_account_rest::dto::response::UserLoggedInSuccessfullyResponse;
use argentum_user_account_rest::dto::schema::LoginResult;
use argentum_user_account_rest::server::handler::UserLoginsWithPasswordTrait;
use argentum_user_business::entity::user::User;
use async_trait::async_trait;
use std::sync::Arc;

pub struct UserLoginsWithPasswordHandler<L>
where
    L: LoggerTrait,
{
    uc: Arc<UserLoginsWithPasswordUc<UniqueId, L>>,
    dto_to_user_logins_with_password_params: Arc<DtoToUserLoginsWithPasswordParams>,
}

impl<L> UserLoginsWithPasswordHandler<L>
where
    L: LoggerTrait,
{
    pub fn new(
        uc: Arc<UserLoginsWithPasswordUc<UniqueId, L>>,
        dto_to_user_logins_with_password_params: Arc<DtoToUserLoginsWithPasswordParams>,
    ) -> Self {
        Self {
            uc,
            dto_to_user_logins_with_password_params,
        }
    }
}

#[async_trait]
impl<L> UserLoginsWithPasswordTrait for UserLoginsWithPasswordHandler<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn handle(
        &self,
        req: UserLoginsWithPasswordRequest,
        user: User<Self::Id>,
    ) -> Result<UserLoginsWithPasswordOperationResponseEnum, HttpError> {
        let guest = match user {
            User::Anonymous(a) => a,
            User::Authenticated(_) => {
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

                Ok(UserLoginsWithPasswordOperationResponseEnum::Status200(
                    UserLoggedInSuccessfullyResponse::new_application_json(dto),
                ))
            }
            Err(LoginError::WrongEmailOrPassword) => Err(HttpError::Unauthorized(
                Unauthorized::new(&format!("{}", LoginError::WrongEmailOrPassword)),
            )),
            Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
                Arc::new(e),
            ))),
        }
    }
}
