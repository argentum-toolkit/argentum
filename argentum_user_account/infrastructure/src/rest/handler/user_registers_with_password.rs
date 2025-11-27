use crate::rest::transformer::DtoToUserRegistersWithPasswordParams;
use argentum_rest_infrastructure::data_type::error::{Conflict, HttpError, InternalServerError};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::use_case::user_registers_with_password::{
    RegistrationError, UserRegistersWithPasswordUc,
};
use argentum_user_account_rest::dto::operation_response_enum::UserRegistersWithPasswordOperationResponseEnum;
use argentum_user_account_rest::dto::request::UserRegistersWithPasswordRequest;
use argentum_user_account_rest::dto::response::UserRegisteredSuccessfullyResponse;
use argentum_user_account_rest::dto::schema::RegistrationWithPasswordResult;
use argentum_user_account_rest::server::handler::UserRegistersWithPasswordTrait;
use argentum_user_business::entity::user::User;
use async_trait::async_trait;
use std::sync::Arc;

pub struct UserRegistersWithPasswordHandler {
    uc: Arc<UserRegistersWithPasswordUc<UniqueId>>,
    dto_to_user_registers_with_password_params: Arc<DtoToUserRegistersWithPasswordParams>,
}

impl UserRegistersWithPasswordHandler {
    pub fn new(
        uc: Arc<UserRegistersWithPasswordUc<UniqueId>>,
        dto_to_user_registers_with_password_params: Arc<DtoToUserRegistersWithPasswordParams>,
    ) -> Self {
        UserRegistersWithPasswordHandler {
            uc,
            dto_to_user_registers_with_password_params,
        }
    }
}

#[async_trait]
impl UserRegistersWithPasswordTrait for UserRegistersWithPasswordHandler {
    type Id = UniqueId;

    async fn handle(
        &self,
        req: UserRegistersWithPasswordRequest,
        _user: User<Self::Id>,
    ) -> Result<UserRegistersWithPasswordOperationResponseEnum, HttpError> {
        let user_id = UniqueId::new();

        let (name, email, password, terms_accepted) = self
            .dto_to_user_registers_with_password_params
            .transform(req)?;

        let result = self
            .uc
            .execute(user_id, name, email, password.into(), terms_accepted)
            .await;

        match result {
            Ok(user) => {
                let dto = RegistrationWithPasswordResult::new(user.id.into());

                Ok(UserRegistersWithPasswordOperationResponseEnum::Status201(
                    UserRegisteredSuccessfullyResponse::new_application_json(dto),
                ))
            }

            Err(e) => match e {
                RegistrationError::TermsNotAccepted => {
                    Err(HttpError::Conflict(Conflict::new(Box::new(e))))
                }
                RegistrationError::EmailAlreadyExists => {
                    Err(HttpError::Conflict(Conflict::new(Box::new(e))))
                }
                _ => Err(HttpError::InternalServerError(InternalServerError::new(
                    Arc::new(e),
                ))),
            },
        }
    }
}
