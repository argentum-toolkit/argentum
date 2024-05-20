use super::schema::ag_user_anonymous;
use super::schema::ag_user_anonymous_binding;
use super::schema::ag_user_authenticated;
use super::schema::ag_user_session;
use chrono::{DateTime, Utc};
use diesel_ulid::DieselUlid as Ulid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = ag_user_authenticated)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthenticatedUserModel {
    pub id: Ulid,
    pub created_at: DateTime<Utc>,
    pub first_name: String,
    //TODO: optional???
    pub last_name: String,
    pub email: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = ag_user_anonymous)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AnonymousUserModel {
    pub id: Ulid,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = ag_user_anonymous_binding)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AnonymousBindingModel {
    pub user_id: Ulid,
    pub anonymous_id: Ulid,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = ag_user_session)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Ulid,
    pub user_id: Ulid,
    pub token: String,
}
