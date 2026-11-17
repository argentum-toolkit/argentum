use jiff_sqlx::Timestamp;
use sqlx::FromRow;

#[derive(FromRow)]
pub(crate) struct MigrationDto {
    pub executed_at: Timestamp,
    pub version: String,
}
