use crate::rest::transformer::DtoToUserLoginsWithPasswordParams;
use argentum_rest_infrastructure::data_type::error::{
    HttpError, InternalServerError, Unauthorized,
};
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_account_business::use_case::user_logins_with_password::{
    LoginError, UserLoginsWithPasswordUc,
};
use argentum_user_account_rest::dto::operation_response_enum::UserLoginsWithPasswordOperationResponseEnum;
use argentum_user_account_rest::dto::request::UserLoginsWithPasswordRequest;
use argentum_user_account_rest::dto::response::UserLoggedInSuccessfullyResponse;
use argentum_user_account_rest::dto::schema::LoginResult;
use argentum_user_account_rest::server::handler::UserLoginsWithPasswordTrait;
use argentum_user_business::entity::user::User;
use std::sync::Arc;

pub struct UserLoginsWithPasswordHandler {
    uc: Arc<UserLoginsWithPasswordUc>,
    id_factory: Arc<UniqueIdFactory>,
    dto_to_user_logins_with_password_params: Arc<DtoToUserLoginsWithPasswordParams>,
}

impl UserLoginsWithPasswordHandler {
    pub fn new(
        uc: Arc<UserLoginsWithPasswordUc>,
        id_factory: Arc<UniqueIdFactory>,
        dto_to_user_logins_with_password_params: Arc<DtoToUserLoginsWithPasswordParams>,
    ) -> Self {
        Self {
            uc,
            id_factory,
            dto_to_user_logins_with_password_params,
        }
    }
}

impl UserLoginsWithPasswordTrait for UserLoginsWithPasswordHandler {
    fn handle(
        &self,
        req: UserLoginsWithPasswordRequest,
        user: User,
    ) -> Result<UserLoginsWithPasswordOperationResponseEnum, HttpError> {
        let anonymous = match user {
            User::Anonymous(a) => a,
            User::Authenticated(_) => {
                return Err(HttpError::Unauthorized(Unauthorized::new(
                    "Endpoint is available only for anonymous users".to_string(),
                )));
            }
        };

        let (email, password) = self
            .dto_to_user_logins_with_password_params
            .transform(req)?;

        let result = self.uc.execute(Some(anonymous), email, password);

        match result {
            Ok(session) => {
                let id = self.id_factory.id_to_uuid(&session.user_id);

                let dto = LoginResult::new(session.token, id);

                Ok(UserLoginsWithPasswordOperationResponseEnum::Status200(
                    UserLoggedInSuccessfullyResponse::new_application_json(dto),
                ))
            }
            Err(LoginError::WrongEmailOrPassword) => Err(HttpError::Unauthorized(
                Unauthorized::new(format!("{}", LoginError::WrongEmailOrPassword)),
            )),
            Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
                Box::new(e),
            ))),
        }
    }
}
