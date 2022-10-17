use crate::db_diesel::model::AuthenticatedUserModel;

use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db_diesel::connection::pg::ConnectionPoolManager;
use argentum_user_business::entity::user::AuthenticatedUser;
use argentum_user_business::repository::user_repository::{
    AuthenticatedUserRepositoryTrait, ExternalUserError,
};

use argentum_standard_business::data_type::email::EmailAddress;
use argentum_user_business::data_type::Name;
use std::sync::Arc;

use diesel::prelude::*;
use diesel::result::Error as DieselError;

pub struct AuthenticatedUserRepository {
    connection: Arc<ConnectionPoolManager>,
    id_factory: Arc<UniqueIdFactory>,
}

impl AuthenticatedUserRepository {
    pub fn new(connection: Arc<ConnectionPoolManager>, id_factory: Arc<UniqueIdFactory>) -> Self {
        AuthenticatedUserRepository {
            connection,
            id_factory,
        }
    }
}

impl AuthenticatedUserRepositoryTrait for AuthenticatedUserRepository {
    fn find(&self, id: &Id) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
        use crate::db_diesel::schema::ag_user_authenticated;

        let conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(ExternalUserError::Authenticated(Some(Box::new(e)))),
        };

        let uid = self.id_factory.id_to_uuid(id);
        let results: Result<AuthenticatedUserModel, diesel::result::Error> =
            ag_user_authenticated::table.find(uid).first(&conn);

        match results {
            Ok(u) => {
                let email = match EmailAddress::new(u.email) {
                    Ok(e) => e,
                    Err(e) => return Err(ExternalUserError::Authenticated(Some(Box::new(e)))),
                };

                let name = match Name::new(u.first_name, Some(u.last_name), None) {
                    Ok(n) => n,
                    Err(e) => return Err(ExternalUserError::Authenticated(Some(Box::new(e)))),
                };

                Ok(Some(AuthenticatedUser::new(
                    &self.id_factory.uuid_to_id(u.id),
                    name,
                    email,
                )))
            }

            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(ExternalUserError::Authenticated(Some(Box::new(e)))),
        }
    }

    fn find_by_email(
        &self,
        email: &EmailAddress,
    ) -> Result<Option<AuthenticatedUser>, ExternalUserError> {
        use crate::db_diesel::schema::ag_user_authenticated;
        use crate::db_diesel::schema::ag_user_authenticated::dsl;

        let conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(ExternalUserError::Authenticated(Some(Box::new(e)))),
        };

        let results = ag_user_authenticated::table
            .filter(dsl::email.eq(email.as_string()))
            .limit(1)
            .load::<AuthenticatedUserModel>(&conn);

        match results {
            Ok(items) => match items.first() {
                Some(item) => {
                    let email = match EmailAddress::new(item.email.to_string()) {
                        Ok(e) => e,
                        Err(e) => return Err(ExternalUserError::Authenticated(Some(Box::new(e)))),
                    };

                    let name = match Name::new(
                        item.first_name.to_string(),
                        Some(item.last_name.to_string()),
                        None,
                    ) {
                        Ok(n) => n,
                        Err(e) => return Err(ExternalUserError::Authenticated(Some(Box::new(e)))),
                    };

                    Ok(Some(AuthenticatedUser {
                        id: self.id_factory.uuid_to_id(item.id),
                        created_at: item.created_at,
                        name,
                        email,
                    }))
                }
                None => Ok(None),
            },
            Err(e) => Err(ExternalUserError::Authenticated(Some(Box::new(e)))),
        }
    }

    fn save(&self, user: &AuthenticatedUser) -> Result<(), ExternalUserError> {
        use crate::db_diesel::schema::ag_user_authenticated;

        let last = match user.name.last.clone() {
            Some(l) => l,
            None => "".to_string(),
        };

        let id = self.id_factory.id_to_uuid(&user.id);
        let new_user = AuthenticatedUserModel {
            id,
            created_at: user.created_at,
            first_name: user.name.first.clone(),
            last_name: last.clone(),
            email: user.email.as_string(),
        };

        let conn = match self.connection.get_pooled_connection() {
            Ok(c) => c,
            Err(e) => return Err(ExternalUserError::Authenticated(Some(Box::new(e)))),
        };

        let result = diesel::insert_into(ag_user_authenticated::table)
            .values(&new_user)
            .execute(&conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ExternalUserError::Authenticated(Some(Box::new(e)))),
        }
    }
}
