use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct GuestBindingDto {
    pub member_id: Uuid,
    pub guest_id: Uuid,
    pub created_at: DateTime<Utc>,
}
