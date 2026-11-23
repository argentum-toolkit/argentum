use jiff_sqlx::Timestamp;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct MessageDto {
    pub id: sqlx::types::Uuid,
    pub user_id: Option<sqlx::types::Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub subject: String,
    pub message: String,
    pub created_at: Timestamp,
}
