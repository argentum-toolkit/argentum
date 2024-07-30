use crate::db::dto::AuthenticatedUserDto;
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db::adapter::DbAdapterError;
use argentum_standard_infrastructure::db::slqx_postgres::SqlxPostgresAdapter;
use argentum_user_business::data_type::builder::NameBuilder;
use argentum_user_business::entity::user::AuthenticatedUser;
use argentum_user_business::repository::user_repository::{
    AuthenticatedUserRepositoryTrait, ExternalUserError,
};
use futures::executor::block_on;
use sqlx::postgres::PgArguments;
use sqlx::query::QueryAs;
use sqlx::Postgres;
use std::sync::Arc;

pub struct AuthenticatedUserRepository {
    adapter: Arc<SqlxPostgresAdapter>,
    id_factory: Arc<UniqueIdFactory>,
}

impl AuthenticatedUserRepository {
    pub fn new(adapter: Arc<SqlxPostgresAdapter>, id_factory: Arc<UniqueIdFactory>) -> Self {
        Self {
            adapter,
            id_factory,
        }
    }

    fn find_one<'q>(
        &'q self,
        query: QueryAs<'q, Postgres, AuthenticatedUserDto, PgArguments>,
    ) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
        let result: Result<Option<AuthenticatedUserDto>, DbAdapterError> =
            block_on(self.adapter.fetch_one(query));

        match result {
            Ok(Some(dto)) => {
                let email = match EmailAddress::try_new(dto.email.clone()) {
                    Ok(e) => e,
                    Err(_) => {
                        return Err(ExternalUserError::Authenticated(Some(Box::new(
                            BrokenStoredData::Email(dto.email.clone()),
                        ))))
                    }
                };

                let name_builder =
                    NameBuilder::new(dto.first_name.clone()).last(dto.last_name.clone());

                let name = match name_builder.try_build() {
                    Ok(n) => n,
                    Err(_) => {
                        return Err(ExternalUserError::Authenticated(Some(Box::new(
                            BrokenStoredData::UserName {
                                first: dto.first_name.clone(),
                                last: dto.last_name.clone(),
                            },
                        ))))
                    }
                };

                //TODO: handle errors instead of unwrap
                Ok(Some(AuthenticatedUser {
                    id: self.id_factory.uuid_to_id(dto.id),
                    created_at: dto.created_at,
                    name,
                    email,
                }))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(ExternalUserError::Authenticated(Some(Box::new(e)))),
        }
    }
}

impl AuthenticatedUserRepositoryTrait for AuthenticatedUserRepository {
    fn find(&self, user_id: &Id) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
        let id = self.id_factory.id_to_uuid(user_id);
        let sql = "SELECT id, created_at, first_name, last_name, email FROM ag_user_authenticated WHERE id = $1 LIMIT 1";
        let query = sqlx::query_as(sql).bind(id);

        self.find_one(query)
    }

    fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
        let sql = "SELECT id, created_at, first_name, last_name, email FROM ag_user_authenticated WHERE email = $1 LIMIT 1";
        let query = sqlx::query_as(sql).bind(email.as_string());

        self.find_one(query)
    }

    fn save(&self, user: &AuthenticatedUser) -> Result<(), ExternalUserError> {
        let id = self.id_factory.id_to_uuid(&user.id);

        let last = user.name.last.as_ref().map(|l| l.to_string());

        let sql = "INSERT INTO ag_user_authenticated (id, created_at, first_name, last_name, email) VALUES ($1, $2, $3, $4, $5)";
        let query = sqlx::query(sql)
            .bind(id)
            .bind(user.created_at)
            .bind(user.name.first.to_string())
            .bind(last)
            .bind(user.email.as_string());

        let result = block_on(self.adapter.exec(query));

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ExternalUserError::Authenticated(Some(Box::new(e)))),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BrokenStoredData {
    #[error("Data broken. Name `{first:?}` `{last:?}` is not valid")]
    UserName { first: String, last: Option<String> },

    #[error("Data broken. Email `{0}` is not valid")]
    Email(String),
}
