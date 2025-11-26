use crate::rest::transformer::DtoToUserLoginsWithPasswordParams;
use argentum_log_business::LoggerTrait;
use argentum_rest_infrastructure::data_type::error::{HttpError, Unauthorized};
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
use argentum_user_account_rest::dto::operation_response_enum::UserLoginsWithPasswordOperationResponseEnum;
use argentum_user_account_rest::dto::request::UserLoginsWithPasswordRequest;
use argentum_user_account_rest::dto::response::UserLoggedInSuccessfullyResponse;
use argentum_user_account_rest::dto::schema::LoginResult;
use argentum_user_account_rest::server::handler::UserLoginsWithPasswordTrait;
use argentum_user_business::entity::user::User;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    refresh_token: Option<String>, //todo: refresh token!!!
}
async fn login_user(
    client_id: &str,
    client_secret: &str,
    username: &str,
    password: &str,
    keycloak_url: &str,
    realm: &str,
) -> Result<TokenResponse, String> {
    let client = Client::new();

    let response = client
        .post(format!(
            "{}/realms/{}/protocol/openid-connect/token",
            keycloak_url, realm
        ))
        .form(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("grant_type", "password"),
            ("username", username),
            ("password", password),
            ("scope", "openid"),
        ])
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let token_info: TokenResponse = response.json().await.unwrap();
        Ok(token_info)
    } else {
        let error_text = response.text().await.unwrap();
        Err(error_text)
    }
}

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

use jsonwebtoken::{TokenData, dangerous::insecure_decode};
#[derive(Debug, Deserialize)]
struct Claims {
    sub: String, // user_id (UUID)
}

fn decode_token_without_verification(
    access_token: &str,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let token_data: TokenData<Claims> = insecure_decode(access_token)?;

    let payload = token_data.claims;

    let user_id = payload.sub;

    Ok(Uuid::parse_str(&user_id).unwrap())
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
        let anonymous = match user {
            User::Anonymous(a) => a,
            User::Authenticated(_) => {
                return Err(HttpError::Unauthorized(Unauthorized::new(
                    "Endpoint is available only for anonymous users",
                )));
            }
        };

        //todo: bind anonymous

        let (email, password) = self
            .dto_to_user_logins_with_password_params
            .transform(req)?;

        let token_response = login_user(
            "argentum-be-client-dev",
            "dj9cobGEQptoEk5gMvqadYHd6emeUQ2Y",
            &email.as_string(),
            &password,
            "http://service.keycloak:8080",
            "argentum-realm-dev",
        )
        .await
        .unwrap();

        let access_token = token_response.access_token.unwrap();

        let user_id = decode_token_without_verification(&access_token).unwrap();

        let dto = LoginResult::new(user_id, access_token.clone());
        // let result = self.uc.execute(Some(anonymous), email, &password).await;
        return Ok(UserLoginsWithPasswordOperationResponseEnum::Status200(
            UserLoggedInSuccessfullyResponse::new_application_json(dto),
        ));

        // match result {
        //     Ok(session) => {
        //         let dto = LoginResult::new(session.user_id.into(), session.token);

        //         Ok(UserLoginsWithPasswordOperationResponseEnum::Status200(
        //             UserLoggedInSuccessfullyResponse::new_application_json(dto),
        //         ))
        //     }
        //     Err(LoginError::WrongEmailOrPassword) => Err(HttpError::Unauthorized(
        //         Unauthorized::new(&format!("{}", LoginError::WrongEmailOrPassword)),
        //     )),
        //     Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
        //         Arc::new(e),
        //     ))),
        // }
    }
}
