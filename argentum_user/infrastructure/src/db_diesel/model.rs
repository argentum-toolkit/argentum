use super::schema::ag_user_anonymous;
use super::schema::ag_user_anonymous_binding;
use super::schema::ag_user_authenticated;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[table_name = "ag_user_authenticated"]
pub struct AuthenticatedUserModel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub first_name: String,
    //TODO: optional???
    pub last_name: String,
    pub email: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "ag_user_anonymous"]
pub struct AnonymousUserModel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Insertable)]
#[table_name = "ag_user_anonymous_binding"]
pub struct AnonymousBindingModel {
    pub user_id: Uuid,
    pub anonymous_id: Uuid,
    pub created_at: DateTime<Utc>,
}
