use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct AuthenticatedUserDto {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub first_name: String,
    //TODO: optional???
    pub last_name: Option<String>,
    pub email: String,
}
