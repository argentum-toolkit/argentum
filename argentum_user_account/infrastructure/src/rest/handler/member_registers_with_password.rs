use crate::rest::transformer::DtoToMemberRegistersWithPasswordParams;
use argentum_rest_infrastructure::data_type::error::{Conflict, HttpError, InternalServerError};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::use_case::member_registers_with_password::{
    MemberRegistersWithPasswordUc, RegistrationError,
};
use argentum_user_account_rest::dto::operation_response_enum::MemberRegistersWithPasswordOperationResponseEnum;
use argentum_user_account_rest::dto::request::MemberRegistersWithPasswordRequest;
use argentum_user_account_rest::dto::response::MemberRegisteredSuccessfullyResponse;
use argentum_user_account_rest::dto::schema::RegistrationWithPasswordResult;
use argentum_user_account_rest::server::handler::MemberRegistersWithPasswordTrait;
use argentum_user_business::entity::user::User;
use async_trait::async_trait;
use std::sync::Arc;

pub struct MemberRegistersWithPasswordHandler {
    uc: Arc<MemberRegistersWithPasswordUc<UniqueId>>,
    dto_to_user_registers_with_password_params: Arc<DtoToMemberRegistersWithPasswordParams>,
}

impl MemberRegistersWithPasswordHandler {
    pub fn new(
        uc: Arc<MemberRegistersWithPasswordUc<UniqueId>>,
        dto_to_user_registers_with_password_params: Arc<DtoToMemberRegistersWithPasswordParams>,
    ) -> Self {
        MemberRegistersWithPasswordHandler {
            uc,
            dto_to_user_registers_with_password_params,
        }
    }
}

#[async_trait]
impl MemberRegistersWithPasswordTrait for MemberRegistersWithPasswordHandler {
    type Id = UniqueId;

    async fn handle(
        &self,
        req: MemberRegistersWithPasswordRequest,
        _user: User<Self::Id>,
    ) -> Result<MemberRegistersWithPasswordOperationResponseEnum, HttpError> {
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

                Ok(MemberRegistersWithPasswordOperationResponseEnum::Status201(
                    MemberRegisteredSuccessfullyResponse::new_application_json(dto),
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
