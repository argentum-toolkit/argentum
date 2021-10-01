use super::schema::ag_user_account_password_credential;
use super::schema::ag_user_account_restore_password_token;
use super::schema::ag_user_account_session;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[table_name = "ag_user_account_session"]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "ag_user_account_restore_password_token"]
pub struct RestorePasswordTokenModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Insertable)]
#[table_name = "ag_user_account_password_credential"]
pub struct PasswordCredentialModel {
    pub user_id: Uuid,
    pub password: String,
    pub salt: String,
}
