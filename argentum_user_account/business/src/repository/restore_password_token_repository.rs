use crate::entity::restore_password_token::RestorePasswordToken;
use argentum_standard_business::data_type::id::Id;

pub trait RestorePasswordTokenRepositoryTrait: Sync + Send {
    fn find(
        &self,
        id: &Id,
    ) -> Result<Option<RestorePasswordToken>, RestorePasswordTokenRepositoryError>;

    fn find_by_token(
        &self,
        token: String,
    ) -> Result<Option<RestorePasswordToken>, RestorePasswordTokenRepositoryError>;

    fn save(&self, token: &RestorePasswordToken)
        -> Result<(), RestorePasswordTokenRepositoryError>;

    fn delete_users_tokens(&self, user_id: &Id) -> Result<(), RestorePasswordTokenRepositoryError>;
}

#[derive(thiserror::Error, Debug)]
pub enum RestorePasswordTokenRepositoryError {
    #[error("Can't save session")]
    Save,

    #[error("Can't delete session")]
    Delete,
}
