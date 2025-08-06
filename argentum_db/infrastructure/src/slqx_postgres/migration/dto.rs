use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(FromRow)]
pub(crate) struct MigrationDto {
    pub executed_at: DateTime<Utc>,
    pub version: String,
}
