use crate::db::ContactUsSqlxPostgresAdapter;
use crate::db::dto::MessageDto;

use argentum_contact_us_business::entity::Message;
use argentum_contact_us_business::repository::message_repository::{
    MessageRepositoryError, MessageRepositoryTrait,
};
use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use async_trait::async_trait;
use jiff_sqlx::ToSqlx;
use sqlx::postgres::{PgArguments, Postgres};
use sqlx::query::QueryAs;
use std::sync::Arc;
use uuid::Uuid;

const TABLE_NAME: &str = "ag_contact_us_message";

pub struct MessageRepository<L>
where
    L: LoggerTrait,
{
    adapter: Arc<ContactUsSqlxPostgresAdapter<L>>,
}

impl<L> MessageRepository<L>
where
    L: LoggerTrait,
{
    pub fn new(adapter: Arc<ContactUsSqlxPostgresAdapter<L>>) -> Self {
        Self { adapter }
    }

    async fn find_one<'q>(
        &'q self,
        query: QueryAs<'q, Postgres, MessageDto, PgArguments>,
    ) -> Result<Option<Message<UniqueId>>, MessageRepositoryError> {
        let result: Result<Option<MessageDto>, _> = self.adapter.fetch_one(query).await;

        match result {
            Ok(Some(dto)) => {
                let email = match dto.email {
                    Some(email) => Some(
                        EmailAddress::try_new(&email)
                            .map_err(|e| MessageRepositoryError::Find(Some(Arc::new(e))))?,
                    ),
                    None => None,
                };

                Ok(Some(Message {
                    id: dto.id.into(),
                    user_id: dto.user_id.map(|id| id.into()),
                    name: dto.name,
                    email,
                    subject: dto.subject,
                    message: dto.message,
                    created_at: dto.created_at.to_jiff(),
                }))
            }

            Ok(None) => Ok(None),
            Err(e) => Err(MessageRepositoryError::Find(Some(Arc::new(e)))),
        }
    }
}

#[async_trait]
impl<L> MessageRepositoryTrait for MessageRepository<L>
where
    L: LoggerTrait,
{
    type Id = UniqueId;

    async fn find(
        &self,
        message_id: Self::Id,
    ) -> Result<Option<Message<Self::Id>>, MessageRepositoryError> {
        let id: Uuid = message_id.into();
        let sql = format!(
            "SELECT id, user_id, name, email, subject, message, created_at \
             FROM {TABLE_NAME} WHERE id = $1 LIMIT 1"
        );
        let query = sqlx::query_as::<_, MessageDto>(sqlx::AssertSqlSafe(sql)).bind(id);

        self.find_one(query).await
    }

    async fn save(&self, message: &Message<Self::Id>) -> Result<(), MessageRepositoryError> {
        let sql = format!(
            "INSERT INTO {TABLE_NAME} (id, subject, message, user_id, name, email, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        );

        let id: Uuid = message.id.into();
        let user_id: Option<Uuid> = message.user_id.as_ref().map(|id| (*id).into());

        let query = sqlx::query(sqlx::AssertSqlSafe(sql))
            .bind(id)
            .bind(message.subject.clone())
            .bind(message.message.clone())
            .bind(user_id)
            .bind(message.name.clone())
            .bind(message.email.as_ref().map(|e| e.as_string()))
            .bind(message.created_at.to_sqlx());

        let result = self.adapter.exec(query).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(MessageRepositoryError::Save(Some(Arc::new(e)))),
        }
    }
}
