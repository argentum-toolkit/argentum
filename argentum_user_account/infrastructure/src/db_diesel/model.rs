use super::schema::ag_user_account_password_credential;
use super::schema::ag_user_account_restore_password_token;
use super::schema::ag_user_account_session;
use chrono::{DateTime, Utc};
use diesel_ulid::DieselUlid as Ulid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = ag_user_account_session)]
pub struct Session {
    pub id: Ulid,
    pub user_id: Ulid,
    pub token: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = ag_user_account_restore_password_token)]
pub struct RestorePasswordTokenModel {
    pub id: Ulid,
    pub user_id: Ulid,
    pub token: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = ag_user_account_password_credential)]
pub struct PasswordCredentialModel {
    pub user_id: Ulid,
    pub password: String,
    pub salt: String,
}
