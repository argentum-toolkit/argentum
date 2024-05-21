use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub(crate) struct AnonymousUserDto {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}
