use crate::db_diesel::model::AnonymousUserModel;

use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db_diesel::connection::pg::ConnectionPoolManager;
use argentum_user_business::entity::user::AnonymousUser;
use argentum_user_business::repository::user_repository::{
    AnonymousUserRepositoryTrait, ExternalUserError,
};

use std::sync::Arc;

use diesel::prelude::*;
use diesel::result::Error as DieselError;

pub struct AnonymousUserRepository {
    connection: Arc<ConnectionPoolManager>,
    id_factory: Arc<UniqueIdFactory>,
}

impl AnonymousUserRepository {
    pub fn new(connection: Arc<ConnectionPoolManager>, id_factory: Arc<UniqueIdFactory>) -> Self {
        AnonymousUserRepository {
            connection,
            id_factory,
        }
    }
}

impl AnonymousUserRepositoryTrait for AnonymousUserRepository {
    fn find(&self, id: &Id) -> Result<Option<AnonymousUser>, ExternalUserError> {
        use crate::db_diesel::schema::ag_user_anonymous;

        let mut conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(ExternalUserError::Anonymous(Some(Box::new(e)))),
        };

        let uid = self.id_factory.id_to_uuid(id);
        let results: Result<AnonymousUserModel, diesel::result::Error> =
            ag_user_anonymous::table.find(uid).first(&mut conn);

        match results {
            Ok(u) => Ok(Some(AnonymousUser::new(
                &self.id_factory.uuid_to_id(u.id.into()),
            ))),

            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(ExternalUserError::Anonymous(Some(Box::new(e)))),
        }
    }

    fn save(&self, user: &AnonymousUser) -> Result<(), ExternalUserError> {
        use crate::db_diesel::schema::ag_user_anonymous;

        let id = self.id_factory.id_to_uuid(&user.id);
        let new_user = AnonymousUserModel {
            id: id.into(),
            created_at: user.created_at,
        };

        let mut conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(ExternalUserError::Anonymous(Some(Box::new(e)))),
        };

        let result = diesel::insert_into(ag_user_anonymous::table)
            .values(&new_user)
            .execute(&mut conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ExternalUserError::Anonymous(Some(Box::new(e)))),
        }
    }
}
