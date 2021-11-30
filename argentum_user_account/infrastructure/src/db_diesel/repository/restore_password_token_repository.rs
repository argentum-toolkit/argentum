use crate::db_diesel::model::RestorePasswordTokenModel;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db_diesel::connection::pg::ConnectionPoolManager;
use argentum_user_account_business::entity::restore_password_token::RestorePasswordToken;
use argentum_user_account_business::repository::restore_password_token_repository::{
    RestorePasswordTokenRepositoryError, RestorePasswordTokenRepositoryTrait,
};
use std::sync::Arc;

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::{QueryDsl, RunQueryDsl};

pub struct RestorePasswordTokenRepository {
    connection: Arc<ConnectionPoolManager>,
    id_factory: Arc<UniqueIdFactory>,
}

impl RestorePasswordTokenRepository {
    pub fn new(
        connection: Arc<ConnectionPoolManager>,
        id_factory: Arc<UniqueIdFactory>,
    ) -> RestorePasswordTokenRepository {
        RestorePasswordTokenRepository {
            connection,
            id_factory,
        }
    }
}

impl RestorePasswordTokenRepositoryTrait for RestorePasswordTokenRepository {
    fn find(
        &self,
        token_id: &Id,
    ) -> Result<Option<RestorePasswordToken>, RestorePasswordTokenRepositoryError> {
        use crate::db_diesel::schema::ag_user_account_restore_password_token;

        let conn = self.connection.get_pooled_connection().unwrap();
        let uuid = self.id_factory.id_to_uuid(token_id);

        let results: Result<RestorePasswordTokenModel, diesel::result::Error> =
            ag_user_account_restore_password_token::table
                .find(uuid)
                .first(&conn);

        match results {
            Ok(t) => Ok(Some(RestorePasswordToken {
                id: self.id_factory.uuid_to_id(t.id),
                user_id: self.id_factory.uuid_to_id(t.user_id),
                token: t.token,
                created_at: t.created_at,
            })),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(RestorePasswordTokenRepositoryError::Other(Some(Box::new(
                e,
            )))),
        }
    }

    fn find_by_token(
        &self,
        token_str: String,
    ) -> Result<Option<RestorePasswordToken>, RestorePasswordTokenRepositoryError> {
        use crate::db_diesel::schema::ag_user_account_restore_password_token;
        use crate::db_diesel::schema::ag_user_account_restore_password_token::dsl;

        let conn = self.connection.get_pooled_connection().unwrap();

        let results = ag_user_account_restore_password_token::table
            .filter(dsl::token.eq(token_str))
            .limit(1)
            .load::<RestorePasswordTokenModel>(&conn);

        match results {
            Ok(items) => match items.first() {
                Some(item) => Ok(Some(RestorePasswordToken {
                    id: self.id_factory.uuid_to_id(item.id),
                    user_id: self.id_factory.uuid_to_id(item.user_id),
                    token: item.token.clone(),
                    created_at: item.created_at,
                })),
                None => Ok(None),
            },
            Err(e) => Err(RestorePasswordTokenRepositoryError::Other(Some(Box::new(
                e,
            )))),
        }
    }

    fn save(
        &self,
        token: &RestorePasswordToken,
    ) -> Result<(), RestorePasswordTokenRepositoryError> {
        use crate::db_diesel::schema::ag_user_account_restore_password_token;

        let id = self.id_factory.id_to_uuid(&token.id);
        let user_id = self.id_factory.id_to_uuid(&token.user_id);
        let new_token = RestorePasswordTokenModel {
            id,
            user_id,
            token: token.token.clone(),
            created_at: token.created_at,
        };

        let conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => {
                return Err(RestorePasswordTokenRepositoryError::Other(Some(Box::new(
                    e,
                ))));
            }
        };

        match diesel::insert_into(ag_user_account_restore_password_token::table)
            .values(&new_token)
            .execute(&conn)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(RestorePasswordTokenRepositoryError::Save(Some(Box::new(e)))),
        }
    }

    fn delete_users_tokens(&self, user_id: &Id) -> Result<(), RestorePasswordTokenRepositoryError> {
        use crate::db_diesel::schema::ag_user_account_restore_password_token::dsl;

        let conn = self.connection.get_pooled_connection().unwrap();

        let result = diesel::delete(
            dsl::ag_user_account_restore_password_token
                .filter(dsl::user_id.eq(self.id_factory.id_to_uuid(user_id))),
        )
        .execute(&conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RestorePasswordTokenRepositoryError::Delete(Some(Box::new(
                e,
            )))),
        }
    }
}
