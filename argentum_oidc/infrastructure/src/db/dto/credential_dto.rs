use jiff_sqlx::Timestamp;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub(crate) struct CredentialDto {
    pub external_id: String,
    pub user_id: Uuid,
    pub provider: String,
    pub created_at: Timestamp,
}
