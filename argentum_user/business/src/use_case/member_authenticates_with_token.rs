use argentum_standard_business::data_type::id::Id;

use crate::entity::user::User;
use crate::entity::user::User::{Guest, Member};
use crate::repository::session_repository::{SessionRepositoryError, SessionRepositoryTrait};
use crate::repository::user_repository::{ExternalUserError, GuestUserRepositoryTrait};
use crate::service::{MemberAuthenticatorError, MemberAuthenticatorTrait, TokenRouterTrait};
use std::sync::Arc;

pub struct MemberAuthenticatesWithTokenUc<I>
where
    I: Id,
{
    guest_repository: Arc<dyn GuestUserRepositoryTrait<Id = I>>,
    session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
    member_authenticator: Arc<dyn MemberAuthenticatorTrait<Id = I>>,
    token_router: Arc<dyn TokenRouterTrait>,
}

impl<I> MemberAuthenticatesWithTokenUc<I>
where
    I: Id,
{
    pub fn new(
        member_authenticator: Arc<dyn MemberAuthenticatorTrait<Id = I>>,
        guest_repository: Arc<dyn GuestUserRepositoryTrait<Id = I>>,
        session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
        token_router: Arc<dyn TokenRouterTrait>,
    ) -> Self {
        Self {
            guest_repository,
            session_repository,
            member_authenticator,
            token_router,
        }
    }

    pub async fn execute(&self, token: &str) -> Result<User<I>, AuthenticationError> {
        if self.token_router.is_guest(token) {
            let session = match self.session_repository.find_by_token(token).await? {
                Some(s) => s,
                None => return Err(AuthenticationError::WrongToken),
            };

            match self.guest_repository.find(session.user_id).await? {
                Some(a) => Ok(Guest(a)),
                None => Err(AuthenticationError::UserNotFound),
            }
        } else {
            match self.member_authenticator.authenticate(token).await? {
                Some(user) => Ok(Member(user)),
                None => Err(AuthenticationError::WrongToken),
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error("User not found")]
    UserNotFound,

    #[error("Wrong token")]
    WrongToken,

    #[error("User repository error")]
    UserRepositoryError(
        #[source]
        #[from]
        ExternalUserError,
    ),

    #[error("Member authenticator error")]
    MemberAuthenticator(
        #[source]
        #[from]
        MemberAuthenticatorError,
    ),

    #[error("Session repository error")]
    SessionRepositoryError(
        #[source]
        #[from]
        SessionRepositoryError,
    ),
}

#[cfg(test)]
mod tests {
    use argentum_standard_business::data_type::EmailAddress;
    use argentum_standard_business::data_type::id::IdFactory;

    use crate::data_type::builder::NameBuilder;
    use crate::entity::session::Session;
    use crate::entity::user::{Member, User};
    use crate::mock::repository::guest_repository_mock::GuestRepositoryMock;
    use crate::mock::repository::member_repository_mock::MemberRepositoryMock;
    use crate::mock::repository::session_repository_mock::SessionRepositoryMock;
    use crate::mock::service::{MemberAuthenticatorMock, TokenRouterMock};
    use crate::repository::session_repository::SessionRepositoryTrait;
    use crate::repository::user_repository::MemberRepositoryTrait;
    use crate::use_case::member_authenticates_with_token::AuthenticationError;
    use crate::use_case::member_authenticates_with_token::MemberAuthenticatesWithTokenUc;
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_authenticates_with_token() -> Result<(), String> {
        let guest_repository = Arc::new(GuestRepositoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let id_factory = IdFactoryMock::new();

        //Data
        let member_id = id_factory.create();
        let session_id = id_factory.create();
        let token = "test-token";
        let authenticated_user = Member::new(
            member_id,
            NameBuilder::new("Dionne")
                .last(Some("Morrison"))
                .try_build()
                .expect("Name should be valid"),
            EmailAddress::try_new("aa@a.com").expect("Email should be valid"),
            true,
        );

        let member_authenticator = Arc::new(MemberAuthenticatorMock::new());
        let _ = member_authenticator.add(token, &authenticated_user);

        let session = Session::new(session_id, member_id, token);

        session_repository
            .save(&session)
            .await
            .expect("Can't save a session");

        let token_router = Arc::new(TokenRouterMock::new());

        //Test
        let uc = MemberAuthenticatesWithTokenUc::new(
            member_authenticator,
            // authenticated_user_repository,
            guest_repository,
            session_repository,
            token_router,
        );

        let result = uc.execute(token).await;

        match result {
            Ok(u) => match u {
                User::Member(m) => {
                    assert_eq!(m.id.to_string(), member_id.clone().to_string());

                    return Ok(());
                }
                User::Guest(_) => {
                    return Err("Return's guest. Not authenticated".into());
                }
            },
            Err(e) => {
                return Err(format!("User is not authenticated: Error: {}", e));
            }
        }
    }

    #[tokio::test]
    async fn test_authenticates_with_token_should_returns_error_if_token_invalid()
    -> Result<(), &'static str> {
        let id_factory = Arc::new(IdFactoryMock::new());
        let guest_repository = Arc::new(GuestRepositoryMock::new());
        let authenticated_user_repository = Arc::new(MemberRepositoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());

        let member_authenticator = Arc::new(MemberAuthenticatorMock::new());

        //Data
        let user_id = id_factory.create();
        let session_id = id_factory.create();
        let token = "test-token";
        let authenticated_user = Member::new(
            user_id,
            NameBuilder::new("Dionne")
                .last(Some("Morrison"))
                .try_build()
                .expect("Name should be valid"),
            EmailAddress::try_new("aa@a.com").expect("Email should be valid"),
            true,
        );
        let session = Session::new(session_id, user_id, token);

        //Prefilling
        authenticated_user_repository
            .create(&authenticated_user)
            .await
            .expect("Can't save a user");
        session_repository
            .save(&session)
            .await
            .expect("Can't save a session");

        //Test
        let token_router = Arc::new(TokenRouterMock::new());

        //Test
        let uc = MemberAuthenticatesWithTokenUc::new(
            member_authenticator,
            // authenticated_user_repository,
            guest_repository,
            session_repository,
            token_router,
        );

        let result = uc.execute("wrong-test-token").await;

        match result {
            Ok(_) => {
                return Err("Shpould return error, not an user");
            }
            Err(e) => match e {
                AuthenticationError::WrongToken => return Ok(()),
                _ => {
                    return Err("Invalid response status");
                }
            },
        }
    }

    #[tokio::test]
    async fn test_authenticates_with_token_should_returns_error_if_user_doesnt_exist()
    -> Result<(), String> {
        let guest_repository = Arc::new(GuestRepositoryMock::new());
        let session_repository = Arc::new(SessionRepositoryMock::new());
        let id_factory = IdFactoryMock::new();

        let member_authenticator = Arc::new(MemberAuthenticatorMock::new());

        //Data
        let user_id = id_factory.create();
        let session_id = id_factory.create();
        let token = "guest";
        let session = Session::new(session_id, user_id, token);
        let token_router = Arc::new(TokenRouterMock::new());

        //Prefilling
        session_repository
            .save(&session)
            .await
            .expect("Can't save a session");

        //Test
        let uc = MemberAuthenticatesWithTokenUc::new(
            member_authenticator,
            // authenticated_user_repository,
            guest_repository,
            session_repository,
            token_router,
        );

        let result = uc.execute(token).await;

        match result {
            Ok(_) => {
                return Err("Should return error, not an user".into());
            }
            Err(e) => match e {
                AuthenticationError::UserNotFound => return Ok(()),
                _ => {
                    return Err(format!("Invalid response status. Error: {}", e));
                }
            },
        }
    }
}
