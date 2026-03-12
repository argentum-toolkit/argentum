use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct MemberDto {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub additional_name: Option<String>,
    pub email: String,
    pub email_confirmed: bool,
}
