use argentum_rest_infrastructure::data_type::error::{
    HttpError, InternalServerError, NotFoundError,
};
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_user_business::entity::user::User;
use argentum_user_business::use_case::{GetUserError, GetUserUc};
use argentum_user_rest::dto::operation_response_enum::GetUserOperationResponseEnum;
use argentum_user_rest::dto::request::GetUserRequest;
use argentum_user_rest::dto::response::GetUserOkResponse;
use argentum_user_rest::dto::schema::User as UserDto;
use argentum_user_rest::dto::schema::UserName as UserNameDto;
use argentum_user_rest::server::handler::GetUserTrait;
use std::sync::Arc;

pub struct GetUserHandler {
    uc: Arc<GetUserUc>,
    id_factory: Arc<UniqueIdFactory>,
}

impl GetUserHandler {
    pub fn new(uc: Arc<GetUserUc>, id_factory: Arc<UniqueIdFactory>) -> Self {
        GetUserHandler { uc, id_factory }
    }
}

impl GetUserTrait for GetUserHandler {
    fn handle(
        &self,
        req: GetUserRequest,
        _user: User,
    ) -> Result<GetUserOperationResponseEnum, HttpError> {
        //TODO: check if authenticated user granted to see this resource
        let user_id = self.id_factory.uuid_to_id(req.params.path.user_id);
        let result = self.uc.execute(user_id);

        match result {
            Ok(user) => {
                let id = self.id_factory.id_to_uuid(&user.id);
                let schema = UserDto::new(
                    user.email.as_string(),
                    Some(id),
                    UserNameDto::new(
                        user.name.first.to_string(),
                        match user.name.last {
                            None => None,
                            Some(l) => Some(l.to_string()),
                        },
                        match user.name.patronymic {
                            None => None,
                            Some(p) => Some(p.to_string()),
                        },
                    ),
                );

                Ok(GetUserOperationResponseEnum::Status200(
                    GetUserOkResponse::new_application_json(schema.clone()),
                ))
            }
            Err(GetUserError::UserNotFound) => {
                Err(HttpError::NotFound(NotFoundError::new(format!(
                    "User with id `{}` is not found",
                    req.params.path.user_id.to_string()
                ))))
            }
            Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
                Box::new(e),
            ))),
        }
    }
}
