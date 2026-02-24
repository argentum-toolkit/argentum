use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::repository::user_repository::MemberRepositoryTrait;
use async_trait::async_trait;
use std::{error::Error, sync::Arc};

#[async_trait]
pub trait MemberConfirmsEmailUcTrait: Send + Sync {
    type Id: Id;

    async fn execute(&self, token: &str) -> Result<(), MemberConfirmsEmailError>;
}

pub struct DefaultMemberConfirmsEmailUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    _user_repository: Arc<dyn MemberRepositoryTrait<Id = I>>,
    _logger: Arc<L>,
}

impl<I, L> DefaultMemberConfirmsEmailUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(_user_repository: Arc<dyn MemberRepositoryTrait<Id = I>>, _logger: Arc<L>) -> Self {
        //TODO: implement
        Self {
            _user_repository,
            _logger,
        }
    }
}

#[async_trait]
impl<I, L> MemberConfirmsEmailUcTrait for DefaultMemberConfirmsEmailUc<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    type Id = I;

    async fn execute(&self, _token: &str) -> Result<(), MemberConfirmsEmailError> {
        todo!();
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MemberConfirmsEmailError {
    #[error("User is not found")]
    UserNotFoundError,

    #[error("Token is not found")]
    TokenNotFoundError,

    #[error("Token expired")]
    TokenExpired,

    #[error("Can't confirm an email.")]
    Network(#[source] Arc<dyn Error + Send + Sync>),

    #[error("Can't confirm an email.")]
    Other(String),
}
