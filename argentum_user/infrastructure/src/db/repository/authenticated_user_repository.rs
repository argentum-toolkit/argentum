// use crate::db::dto::AuthenticatedUserDto;
// use argentum_db_infrastructure::adapter::DbAdapterError;
// use argentum_db_infrastructure::slqx_postgres::SqlxPostgresAdapter;
// use argentum_log_business::LoggerTrait;
// use argentum_standard_business::data_type::EmailAddress;
// use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
// use argentum_user_business::data_type::builder::NameBuilder;
// use argentum_user_business::entity::user::AuthenticatedUser;
// use argentum_user_business::repository::user_repository::{
//     AuthenticatedUserRepositoryTrait, ExternalUserError,
// };
// use sqlx::Postgres;
// use sqlx::postgres::PgArguments;
// use sqlx::query::QueryAs;
// use std::sync::Arc;
// use uuid::Uuid;

// use async_trait::async_trait;

// const TABLE_NAME: &str = "ag_user_authenticated";

// pub struct AuthenticatedUserRepository<L>
// where
//     L: LoggerTrait,
// {
//     adapter: Arc<SqlxPostgresAdapter<L>>,
// }

// impl<L> AuthenticatedUserRepository<L>
// where
//     L: LoggerTrait,
// {
//     pub fn new(adapter: Arc<SqlxPostgresAdapter<L>>) -> Self {
//         Self { adapter }
//     }

//     async fn find_one<'q>(
//         &'q self,
//         query: QueryAs<'q, Postgres, AuthenticatedUserDto, PgArguments>,
//     ) -> Result<Option<AuthenticatedUser<UniqueId>>, ExternalUserError> {
//         let result: Result<Option<AuthenticatedUserDto>, DbAdapterError> =
//             self.adapter.fetch_one(query).await;

//         match result {
//             Ok(Some(dto)) => {
//                 let email = match EmailAddress::try_new(&dto.email) {
//                     Ok(e) => e,
//                     Err(_) => {
//                         return Err(ExternalUserError::Authenticated(Some(Arc::new(
//                             BrokenStoredData::Email(dto.email.clone()),
//                         ))));
//                     }
//                 };

//                 let name_builder =
//                     NameBuilder::new(&dto.first_name).last(dto.last_name.clone().as_deref());

//                 let name = match name_builder.try_build() {
//                     Ok(n) => n,
//                     Err(_) => {
//                         return Err(ExternalUserError::Authenticated(Some(Arc::new(
//                             BrokenStoredData::UserName {
//                                 first: dto.first_name.clone(),
//                                 last: dto.last_name,
//                             },
//                         ))));
//                     }
//                 };

//                 //TODO: handle errors instead of unwrap
//                 Ok(Some(AuthenticatedUser {
//                     id: dto.id.into(),
//                     created_at: dto.created_at,
//                     name,
//                     email,
//                 }))
//             }
//             Ok(None) => Ok(None),
//             Err(e) => Err(ExternalUserError::Authenticated(Some(Arc::new(e)))),
//         }
//     }
// }

// #[async_trait]
// impl<L> AuthenticatedUserRepositoryTrait for AuthenticatedUserRepository<L>
// where
//     L: LoggerTrait,
// {
//     type Id = UniqueId;

//     async fn find(
//         &self,
//         user_id: Self::Id,
//     ) -> Result<Option<AuthenticatedUser<Self::Id>>, ExternalUserError> {
//         let id: Uuid = user_id.into();

//         let sql = format!(
//             "SELECT id, created_at, first_name, last_name, email FROM {TABLE_NAME} WHERE id = $1 LIMIT 1"
//         );
//         let query = sqlx::query_as(&sql).bind(id);

//         self.find_one(query).await
//     }

//     async fn find_by_email(
//         &self,
//         email: &EmailAddress,
//     ) -> Result<Option<AuthenticatedUser<Self::Id>>, ExternalUserError> {
//         let sql = format!(
//             "SELECT id, created_at, first_name, last_name, email FROM {TABLE_NAME} WHERE email = $1 LIMIT 1"
//         );
//         let query = sqlx::query_as(&sql).bind(email.as_string());

//         self.find_one(query).await
//     }

//     async fn create(&self, user: &AuthenticatedUser<Self::Id>) -> Result<(), ExternalUserError> {
//         let last = user.name.last.as_ref().map(|l| l.to_string());
//         let user_id: Uuid = user.id.into();

//         let sql = format!(
//             "INSERT INTO {TABLE_NAME} (id, created_at, first_name, last_name, email) VALUES ($1, $2, $3, $4, $5)"
//         );
//         let query = sqlx::query(&sql)
//             .bind(user_id)
//             .bind(user.created_at)
//             .bind(user.name.first.to_string())
//             .bind(last)
//             .bind(user.email.as_string());

//         let result = self.adapter.exec(query).await;

//         match result {
//             Ok(_) => Ok(()),
//             Err(e) => Err(ExternalUserError::Authenticated(Some(Arc::new(e)))),
//         }
//     }
// }

// #[derive(thiserror::Error, Debug)]
// pub enum BrokenStoredData {
//     #[error("Data broken. Name `{first:?}` `{last:?}` is not valid")]
//     UserName { first: String, last: Option<String> },

//     #[error("Data broken. Email `{0}` is not valid")]
//     Email(String),
// }
