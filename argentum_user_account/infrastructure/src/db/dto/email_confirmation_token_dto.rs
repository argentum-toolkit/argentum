use jiff_sqlx::Timestamp;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct EmailConfirmationTokenDto {
    pub id: sqlx::types::Uuid,
    pub user_id: sqlx::types::Uuid,
    pub token: String,
    pub created_at: Timestamp,
}
