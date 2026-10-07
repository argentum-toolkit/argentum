use crate::db::UserSqlxPostgresAdapter;
use crate::db::dto::MemberDto;
use argentum_db_infrastructure::adapter::DbAdapterError;
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_business::data_type::builder::NameBuilder;
use argentum_user_business::entity::user::Member;
use argentum_user_business::repository::user_repository::{
    ExternalUserError, MemberRepositoryTrait,
};
use sqlx::Postgres;
use sqlx::postgres::PgArguments;
use sqlx::query::QueryAs;
use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;

const TABLE_NAME: &str = "ag_user_member";

pub struct MemberRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<UserSqlxPostgresAdapter<L>>,
}

impl<L> MemberRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(adapter: Arc<UserSqlxPostgresAdapter<L>>) -> Self {
        Self { adapter }
    }

    async fn find_one<'q>(
        &'q self,
        query: QueryAs<'q, Postgres, MemberDto, PgArguments>,
    ) -> Result<Option<Member<UniqueId>>, ExternalUserError> {
        let result: Result<Option<MemberDto>, DbAdapterError> = self.adapter.fetch_one(query).await;

        match result {
            Ok(Some(dto)) => {
                let email = match EmailAddress::try_new(&dto.email) {
                    Ok(e) => e,
                    Err(_) => {
                        return Err(ExternalUserError::Member(Some(Arc::new(
                            BrokenStoredData::Email(dto.email.clone()),
                        ))));
                    }
                };

                let name_builder = NameBuilder::new(&dto.first_name)
                    .last(dto.last_name.clone().as_deref())
                    .additional(dto.additional_name.clone().as_deref());

                let name = match name_builder.try_build() {
                    Ok(n) => n,
                    Err(_) => {
                        return Err(ExternalUserError::Member(Some(Arc::new(
                            BrokenStoredData::UserName {
                                first: dto.first_name.clone(),
                                last: dto.last_name,
                            },
                        ))));
                    }
                };

                Ok(Some(Member {
                    id: dto.id.into(),
                    created_at: dto.created_at,
                    name,
                    email,
                    email_confirmed: dto.email_confirmed,
                }))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(ExternalUserError::Member(Some(Arc::new(e)))),
        }
    }
}

#[async_trait]
impl<L> MemberRepositoryTrait for MemberRepository<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn find(&self, user_id: Self::Id) -> Result<Option<Member<Self::Id>>, ExternalUserError> {
        let id: Uuid = user_id.into();

        let sql = format!(
            "SELECT id, created_at, first_name, last_name, additional_name, email, email_confirmed FROM {TABLE_NAME} WHERE id = $1 LIMIT 1"
        );
        let query = sqlx::query_as(&sql).bind(id);

        self.find_one(query).await
    }

    async fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<Member<Self::Id>>, ExternalUserError> {
        let sql = format!(
            "SELECT id, created_at, first_name, last_name, additional_name, email, email_confirmed FROM {TABLE_NAME} WHERE email = $1 LIMIT 1"
        );
        let query = sqlx::query_as(&sql).bind(email.as_string());

        self.find_one(query).await
    }

    async fn create(&self, member: &Member<Self::Id>) -> Result<(), ExternalUserError> {
        let last = member.name.last.as_ref().map(|l| l.to_string());
        let additional = member.name.additional.as_ref().map(|a| a.to_string());
        let member_id: Uuid = member.id.into();

        let sql = format!(
            "INSERT INTO {TABLE_NAME} (id, created_at, first_name, last_name, additional_name, email, email_confirmed) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        );

        let query = sqlx::query(&sql)
            .bind(member_id)
            .bind(member.created_at)
            .bind(member.name.first.to_string())
            .bind(last)
            .bind(additional)
            .bind(member.email.as_string())
            .bind(member.email_confirmed);

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ExternalUserError::Member(Some(Arc::new(e)))),
        }
    }

    async fn update(&self, member: &Member<Self::Id>) -> Result<(), ExternalUserError> {
        let last = member.name.last.as_ref().map(|l| l.to_string());
        let additional = member.name.additional.as_ref().map(|a| a.to_string());
        let member_id: Uuid = member.id.into();

        let sql = format!(
            "UPDATE {TABLE_NAME} SET first_name = $1, last_name = $2, additional_name = $3, email = $4, email_confirmed = $5 WHERE id = $6"
        );

        let query = sqlx::query(&sql)
            .bind(member.name.first.to_string())
            .bind(last)
            .bind(additional)
            .bind(member.email.as_string())
            .bind(member.email_confirmed)
            .bind(member_id);

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ExternalUserError::Member(Some(Arc::new(e)))),
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
