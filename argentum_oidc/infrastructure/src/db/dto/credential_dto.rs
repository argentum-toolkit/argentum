use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub(crate) struct CredentialDto {
    pub external_id: String,
    pub user_id: Uuid,
    pub provider: String,
    pub created_at: DateTime<Utc>,
}
