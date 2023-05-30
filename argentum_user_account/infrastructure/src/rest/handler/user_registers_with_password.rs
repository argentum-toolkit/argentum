use crate::api::dto::request::UserRegistersWithPasswordRequest;
use crate::api::server::handler::UserRegistersWithPasswordTrait;
use crate::rest::transformer::DtoToUserRegistersWithPasswordParams;
use argentum_rest_infrastructure::data_type::error::{
    BadRequestError, Conflict, HttpError, InternalServerError,
};
use argentum_rest_infrastructure::data_type::HttpResponse;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_standard_business::invariant_violation::Violations;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_account_api::models::RegistrationWithPasswordResult;
use argentum_user_account_business::use_case::user_registers_with_password::{
    RegistrationError, UserRegistersWithPasswordUc,
};
use hyper::StatusCode;
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
    fn handle(&self, req: UserRegistersWithPasswordRequest) -> Result<HttpResponse, HttpError> {
        let user_id = self.id_factory.create();

        let (name, email, password) = match self
            .dto_to_user_registers_with_password_params
            .transform(req)
        {
            Ok(d) => d,
            Err(v) => {
                return Err(HttpError::BadRequest(BadRequestError::new(
                    v,
                    Violations::new(vec![], None),
                )))
            }
        };

        let result = self.uc.execute(user_id, name, email, password);

        match result {
            Ok(user) => {
                let id = self.id_factory.id_to_uuid(&user.id);
                let schema = RegistrationWithPasswordResult::new(id);

                Ok(HttpResponse::new(StatusCode::CREATED, Box::new(schema)))
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
