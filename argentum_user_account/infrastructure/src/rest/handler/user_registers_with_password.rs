use crate::rest::transformer::DtoToUserRegistersWithPasswordParams;
use argentum_rest_infrastructure::data_type::error::{Conflict, HttpError, InternalServerError};
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_account_business::use_case::user_registers_with_password::{
    RegistrationError, UserRegistersWithPasswordUc,
};
use argentum_user_account_rest::dto::operation_response_enum::UserRegistersWithPasswordOperationResponseEnum;
use argentum_user_account_rest::dto::request::UserRegistersWithPasswordRequest;
use argentum_user_account_rest::dto::response::UserRegisteredSuccessfullyResponse;
use argentum_user_account_rest::dto::schema::RegistrationWithPasswordResult;
use argentum_user_account_rest::server::handler::UserRegistersWithPasswordTrait;
use argentum_user_business::entity::user::User;
use std::sync::Arc;

pub struct UserRegistersWithPasswordHandler {
    uc: Arc<UserRegistersWithPasswordUc>,
    id_factory: Arc<UniqueIdFactory>,
    dto_to_user_registers_with_password_params: Arc<DtoToUserRegistersWithPasswordParams>,
}

impl UserRegistersWithPasswordHandler {
    pub fn new(
        uc: Arc<UserRegistersWithPasswordUc>,
        id_factory: Arc<UniqueIdFactory>,
        dto_to_user_registers_with_password_params: Arc<DtoToUserRegistersWithPasswordParams>,
    ) -> Self {
        UserRegistersWithPasswordHandler {
            uc,
            id_factory,
            dto_to_user_registers_with_password_params,
        }
    }
}

impl UserRegistersWithPasswordTrait for UserRegistersWithPasswordHandler {
    fn handle(
        &self,
        req: UserRegistersWithPasswordRequest,
        _user: User,
    ) -> Result<UserRegistersWithPasswordOperationResponseEnum, HttpError> {
        let user_id = self.id_factory.create();

        let (name, email, password) = self
            .dto_to_user_registers_with_password_params
            .transform(req)?;

        let result = self.uc.execute(user_id, name, email, password);

        match result {
            Ok(user) => {
                let id = self.id_factory.id_to_uuid(&user.id);
                let dto = RegistrationWithPasswordResult::new(id);

                Ok(UserRegistersWithPasswordOperationResponseEnum::Status201(
                    UserRegisteredSuccessfullyResponse::new_application_json(dto),
                ))
            }

            Err(e) => match e {
                RegistrationError::EmailAlreadyExists => {
                    Err(HttpError::Conflict(Conflict::new(Box::new(e))))
                }
                _ => Err(HttpError::InternalServerError(InternalServerError::new(
                    Box::new(e),
                ))),
            },
        }
    }
}
