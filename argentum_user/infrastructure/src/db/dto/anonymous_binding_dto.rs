use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct AnonymousBindingDto {
    pub user_id: Uuid,
    pub anonymous_id: Uuid,
    pub created_at: DateTime<Utc>,
}
