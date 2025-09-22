use argentum_rest_infrastructure::data_type::error::{
    HttpError, InternalServerError, NotFoundError,
};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::entity::user::User;
use argentum_user_business::use_case::{GetUserError, GetUserUc};
use argentum_user_rest::dto::operation_response_enum::GetUserOperationResponseEnum;
use argentum_user_rest::dto::request::GetUserRequest;
use argentum_user_rest::dto::response::GetUserOkResponse;
use argentum_user_rest::dto::schema::User as UserDto;
use argentum_user_rest::dto::schema::UserName as UserNameDto;
use argentum_user_rest::server::handler::GetUserTrait;

use async_trait::async_trait;

use std::sync::Arc;

pub struct GetUserHandler {
    uc: Arc<GetUserUc<UniqueId>>,
}

impl GetUserHandler {
    pub fn new(uc: Arc<GetUserUc<UniqueId>>) -> Self {
        GetUserHandler { uc }
    }
}

#[async_trait]
impl GetUserTrait for GetUserHandler {
    type Id = UniqueId;

    async fn handle(
        &self,
        req: GetUserRequest,
        _user: User<Self::Id>,
    ) -> Result<GetUserOperationResponseEnum, HttpError> {
        //TODO: check if authenticated user granted to see this resource
        let user_id = req.params.path.user_id.into();
        let result = self.uc.execute(user_id).await;

        match result {
            Ok(user) => {
                let id = user.id.into();
                let schema = UserDto::new(
                    user.email.as_string(),
                    Some(id),
                    UserNameDto::new(
                        user.name.first.to_string(),
                        user.name.last.map(|l| l.to_string()),
                        user.name.additional.map(|a| a.to_string()),
                    ),
                );

                Ok(GetUserOperationResponseEnum::Status200(
                    GetUserOkResponse::new_application_json(schema.clone()),
                ))
            }
            Err(GetUserError::UserNotFound) => Err(HttpError::NotFound(NotFoundError::new(
                format!("User with id `{}` is not found", req.params.path.user_id),
            ))),
            Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
                Arc::new(e),
            ))),
        }
    }
}
