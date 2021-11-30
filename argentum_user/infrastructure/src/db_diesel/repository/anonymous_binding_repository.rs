use crate::db_diesel::model::AnonymousBindingModel;

use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db_diesel::connection::pg::ConnectionPoolManager;

use argentum_user_business::entity::anonymous_binding::AnonymousBinding;
use argentum_user_business::repository::anonymous_binding_repository::{
    AnonymousBindingRepositoryError, AnonymousBindingRepositoryTrait,
};
use std::sync::Arc;

use diesel::prelude::*;
use diesel::result::Error as DieselError;

pub struct AnonymousBindingRepository {
    connection: Arc<ConnectionPoolManager>,
    id_factory: Arc<UniqueIdFactory>,
}

impl AnonymousBindingRepository {
    pub fn new(connection: Arc<ConnectionPoolManager>, id_factory: Arc<UniqueIdFactory>) -> Self {
        AnonymousBindingRepository {
            connection,
            id_factory,
        }
    }
}

impl AnonymousBindingRepositoryTrait for AnonymousBindingRepository {
    fn find_by_user_id(
        &self,
        id: &Id,
    ) -> Result<Option<AnonymousBinding>, AnonymousBindingRepositoryError> {
        use crate::db_diesel::schema::ag_user_anonymous_binding;

        let conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(AnonymousBindingRepositoryError::Find(Some(Box::new(e)))),
        };

        let uid = self.id_factory.id_to_uuid(id);
        let results: Result<AnonymousBindingModel, diesel::result::Error> =
            ag_user_anonymous_binding::table.find(uid).first(&conn);

        match results {
            Ok(b) => Ok(Some(AnonymousBinding::new(
                self.id_factory.uuid_to_id(b.user_id),
                self.id_factory.uuid_to_id(b.anonymous_id),
            ))),

            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(AnonymousBindingRepositoryError::Find(Some(Box::new(e)))),
        }
    }

    fn save(&self, binding: &AnonymousBinding) -> Result<(), AnonymousBindingRepositoryError> {
        use crate::db_diesel::schema::ag_user_anonymous_binding;

        let user_id = self.id_factory.id_to_uuid(&binding.user_id);
        let anonymous_id = self.id_factory.id_to_uuid(&binding.anonymous_id);
        let new_binding = AnonymousBindingModel {
            user_id,
            anonymous_id,
            created_at: binding.created_at,
        };

        let conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(AnonymousBindingRepositoryError::Save(Some(Box::new(e)))),
        };

        let result = diesel::insert_into(ag_user_anonymous_binding::table)
            .values(&new_binding)
            .execute(&conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(AnonymousBindingRepositoryError::Save(Some(Box::new(e)))),
        }
    }
}
