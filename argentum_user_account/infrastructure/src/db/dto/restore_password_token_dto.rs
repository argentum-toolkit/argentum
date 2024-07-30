use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct RestorePasswordTokenDto {
    pub id: sqlx::types::Uuid,
    pub user_id: sqlx::types::Uuid,
    pub token: String,
    pub created_at: DateTime<Utc>,
}
