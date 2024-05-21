use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub(crate) struct SessionDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
}
