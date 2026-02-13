use crate::data_type::AuthHeaderParams;
use crate::data_type::error::{HttpError, InternalServerError, Unauthorized};
// use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::data_type::id::Id;
// use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
// use argentum_user_business::data_type::{Name, NamePart};
use argentum_user_business::entity::user::User;
use argentum_user_business::use_case::user_authenticates_with_token::AuthenticationError;
use argentum_user_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
// use openidconnect::AccessToken;
// use openidconnect::core::CoreTokenIntrospectionResponse;
// use serde_json::Value;
use std::sync::Arc;
// use uuid::Uuid;

pub struct BearerAuthenticator<I>
where
    I: Id,
{
    uc: Arc<UserAuthenticatesWithTokenUc<I>>,
}

impl<I> BearerAuthenticator<I>
where
    I: Id,
{
    pub fn new(uc: Arc<UserAuthenticatesWithTokenUc<I>>) -> Self {
        Self { uc }
    }

    pub async fn auth(&self, headers: &AuthHeaderParams) -> Result<User<I>, HttpError> {
        let header = headers.authorization.clone();

        match header.find("Bearer ") {
            Some(0) => (),
            _ => {
                return Err(HttpError::Unauthorized(Unauthorized::new(
                    "The expected format of `Authorization` header is `Bearer {token}`",
                )));
            }
        }

        let token: String = header.chars().skip(7).collect();

        match self.uc.execute(&token).await {
            Ok(user) => Ok(user),
            Err(AuthenticationError::UserNotFound) | Err(AuthenticationError::WrongToken) => Err(
                HttpError::Unauthorized(Unauthorized::new("can't authenticate")),
            ),
            Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
                Arc::new(e),
            ))),
        }

        // let token_data = jsonwebtoken::decode_header(&token);
        // match token_data {
        //     Ok(_) => match self.uc.execute(&token).await {
        //         Ok(user) => Ok(user),
        //         Err(AuthenticationError::UserNotFound) | Err(AuthenticationError::WrongToken) => {
        //             Err(HttpError::Unauthorized(Unauthorized::new(
        //                 "can't authenticate",
        //             )))
        //         }
        //         Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
        //             Arc::new(e),
        //         ))),
        //     },
        //     Err(_) => {
        //         let client = reqwest::Client::new();

        //         let res = client
        //             .post("http://service.keycloak:8080/realms/argentum-realm/protocol/openid-connect/token/introspect")
        //             .basic_auth("argentum-be-dev", Some("dj9cobGEQptoEk5gMvqadYHd6emeUQ2Y"))
        //             .form(&[("token", token.clone())])
        //             .send()
        //             .await.unwrap();

        //         let json: Value = res.json().await.unwrap();

        //         println!("{:?}", json);

        //         todo!();
        //     }
        // }

        // let res = reqwest::Client::new()
        // .get("http://service.keycloak:8080/realms/argentum-realm/protocol/openid-connect/userinfo")
        // .bearer_auth(token)
        // .send()
        // .await.unwrap();

        // let data: Value = res.json().await.unwrap();

        // let uid = Uuid::from(data["sub"].as_str().unwrap().into());
        // let id: I = UniqueId::from(uid).as_id();
        // let last_name = data["family_name"].as_str().unwrap();
        // let first_name = data["given_name"].as_str().unwrap();

        // let first = NamePart::try_new(first_name.into()).expect("Should be valid");
        // let last = NamePart::try_new(last_name.into()).expect("Should be valid");

        // let email =
        //     EmailAddress::try_new(data["email"].as_str().unwrap().into()).expect("Should be valid");

        // let name = Name::new(first, Some(last), None);
        // let user: User<I> = User::Authenticated(AuthenticatedUser::new(id, name, email));

        // Ok(user)
        // println!("{:?}", data);

        // match self.uc.execute(&token).await {
        //     Ok(user) => Ok(user),
        //     Err(AuthenticationError::UserNotFound) | Err(AuthenticationError::WrongToken) => {
        //         let client = reqwest::Client::new();

        //         let res = client
        //             .post("http://service.keycloak:8080/realms/argentum-realm/protocol/openid-connect/token/introspect")
        //             .basic_auth("argentum-be-dev", Some("dj9cobGEQptoEk5gMvqadYHd6emeUQ2Y"))
        //             .form(&[("token", token.clone())])
        //             .send()
        //             .await.unwrap();

        //         let json: Value = res.json().await.unwrap();

        //         println!("{:?}", json);

        //         Err(HttpError::Unauthorized(Unauthorized::new(
        //             "can't authenticate",
        //         )))
        //     }
        //     Err(e) => Err(HttpError::InternalServerError(InternalServerError::new(
        //         Arc::new(e),
        //     ))),
        // }
    }
}
