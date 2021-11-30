use crate::db_diesel::model::Session as SessionDbModel;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db_diesel::connection::pg::ConnectionPoolManager;
use argentum_user_account_business::entity::session::Session;
use argentum_user_account_business::repository::session_repository::SessionRepositoryError::Other;
use argentum_user_account_business::repository::session_repository::{
    SessionRepositoryError, SessionRepositoryTrait,
};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::{QueryDsl, RunQueryDsl};
use std::sync::Arc;

pub struct SessionRepository {
    connection: Arc<ConnectionPoolManager>,
    id_factory: Arc<UniqueIdFactory>,
}

impl SessionRepository {
    pub fn new(
        connection: Arc<ConnectionPoolManager>,
        id_factory: Arc<UniqueIdFactory>,
    ) -> SessionRepository {
        SessionRepository {
            connection,
            id_factory,
        }
    }
}

impl SessionRepositoryTrait for SessionRepository {
    fn find(&self, session_id: &Id) -> Result<Option<Session>, SessionRepositoryError> {
        use crate::db_diesel::schema::ag_user_account_session;

        let conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(Other(Some(Box::new(e)))),
        };

        let sid = self.id_factory.id_to_uuid(session_id);
        let results: Result<SessionDbModel, diesel::result::Error> =
            ag_user_account_session::table.find(sid).first(&conn);

        match results {
            Ok(s) => Ok(Some(Session::new(
                self.id_factory.uuid_to_id(s.id),
                self.id_factory.uuid_to_id(s.user_id),
                s.token,
            ))),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Other(Some(Box::new(e)))),
        }
    }

    fn find_by_token(&self, token_str: String) -> Result<Option<Session>, SessionRepositoryError> {
        use crate::db_diesel::schema::ag_user_account_session;
        use crate::db_diesel::schema::ag_user_account_session::dsl;

        let conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(Other(Some(Box::new(e)))),
        };

        let results = ag_user_account_session::table
            .filter(dsl::token.eq(token_str))
            .limit(1)
            .load::<SessionDbModel>(&conn);

        match results {
            Ok(items) => match items.first() {
                Some(item) => Ok(Some(Session {
                    id: self.id_factory.uuid_to_id(item.id),
                    user_id: self.id_factory.uuid_to_id(item.user_id),
                    token: "".to_string(),
                })),
                None => Ok(None),
            },
            Err(e) => Err(Other(Some(Box::new(e)))),
        }
    }

    fn save(&self, session: &Session) -> Result<(), SessionRepositoryError> {
        use crate::db_diesel::schema::ag_user_account_session;

        let id = self.id_factory.id_to_uuid(&session.id);
        let user_id = self.id_factory.id_to_uuid(&session.user_id);
        let new_session = SessionDbModel {
            id,
            user_id,
            token: session.token.clone(),
        };

        let conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(Other(Some(Box::new(e)))),
        };

        match diesel::insert_into(ag_user_account_session::table)
            .values(&new_session)
            .execute(&conn)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(SessionRepositoryError::Save(Some(Box::new(e)))),
        }
    }

    fn delete_users_sessions(&self, user_id: &Id) -> Result<(), SessionRepositoryError> {
        use crate::db_diesel::schema::ag_user_account_session::dsl;

        let conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(Other(Some(Box::new(e)))),
        };

        let result = diesel::delete(
            dsl::ag_user_account_session
                .filter(dsl::user_id.eq(self.id_factory.id_to_uuid(user_id))),
        )
        .execute(&conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(SessionRepositoryError::Delete(Some(Box::new(e)))),
        }
    }
}
