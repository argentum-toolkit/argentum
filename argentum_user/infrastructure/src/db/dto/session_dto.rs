use jiff_sqlx::Timestamp;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub(crate) struct SessionDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: Timestamp,
    pub token: String,
}
