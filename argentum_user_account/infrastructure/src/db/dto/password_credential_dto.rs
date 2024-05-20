use sqlx::FromRow;

#[derive(FromRow)]
pub struct PasswordCredentialDto {
    pub user_id: sqlx::types::Uuid,
    pub password: String,
    pub salt: String,
}
