use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub(crate) struct MigrationDto {
    pub executed_at: DateTime<Utc>,
    pub version: String,
}
