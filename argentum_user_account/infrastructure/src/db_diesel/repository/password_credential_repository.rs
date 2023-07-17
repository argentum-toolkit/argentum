use crate::db_diesel::model::PasswordCredentialModel;
use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db_diesel::connection::pg::ConnectionPoolManager;
use argentum_user_account_business::entity::credential::PasswordCredential;
use argentum_user_account_business::repository::password_credential_repository::{
    PasswordCredentialRepositoryError, PasswordCredentialRepositoryTrait,
};
use std::sync::Arc;

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::{QueryDsl, RunQueryDsl};
use diesel_ulid::DieselUlid;

pub struct PasswordCredentialRepository {
    connection: Arc<ConnectionPoolManager>,
    id_factory: Arc<UniqueIdFactory>,
}

impl PasswordCredentialRepository {
    pub fn new(connection: Arc<ConnectionPoolManager>, id_factory: Arc<UniqueIdFactory>) -> Self {
        PasswordCredentialRepository {
            connection,
            id_factory,
        }
    }
}

impl PasswordCredentialRepositoryTrait for PasswordCredentialRepository {
    fn save(&self, cred: &PasswordCredential) -> Result<(), PasswordCredentialRepositoryError> {
        use crate::db_diesel::schema::ag_user_account_password_credential;

        let user_id = self.id_factory.id_to_uuid(&cred.user_id);
        let new_cred = PasswordCredentialModel {
            user_id: user_id.into(),
            password: cred.password.clone(),
            salt: cred.salt.clone(),
        };

        let mut conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(PasswordCredentialRepositoryError::Other(Some(Box::new(e)))),
        };

        let result = diesel::insert_into(ag_user_account_password_credential::table)
            .values(&new_cred)
            .execute(&mut conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PasswordCredentialRepositoryError::Save(Box::new(e))),
        }
    }

    fn find_by_user_id(
        &self,
        id: &Id,
    ) -> Result<Option<PasswordCredential>, PasswordCredentialRepositoryError> {
        use crate::db_diesel::schema::ag_user_account_password_credential;

        let mut conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(PasswordCredentialRepositoryError::Other(Some(Box::new(e)))),
        };

        let uuid = self.id_factory.id_to_uuid(id);

        let results: Result<PasswordCredentialModel, diesel::result::Error> =
            ag_user_account_password_credential::table
                .find(Into::<DieselUlid>::into(uuid))
                .first(&mut conn);

        match results {
            Ok(c) => Ok(Some(PasswordCredential {
                user_id: self.id_factory.uuid_to_id(c.user_id.into()),
                password: c.password.clone(),
                salt: c.salt,
            })),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(PasswordCredentialRepositoryError::Find(Box::new(e))),
        }
    }

    fn delete(&self, cred: &PasswordCredential) -> Result<(), PasswordCredentialRepositoryError> {
        use crate::db_diesel::schema::ag_user_account_password_credential::dsl;

        let mut conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(PasswordCredentialRepositoryError::Other(Some(Box::new(e)))),
        };

        let result = diesel::delete(dsl::ag_user_account_password_credential.filter(
            dsl::user_id.eq(Into::<DieselUlid>::into(
                self.id_factory.id_to_uuid(&cred.user_id.clone()),
            )),
        ))
        .execute(&mut conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PasswordCredentialRepositoryError::Delete(Box::new(e))),
        }
    }
}
