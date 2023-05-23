use crate::db_diesel::model::AuthenticatedUserModel;

use argentum_standard_business::data_type::id::Id;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db_diesel::connection::pg::ConnectionPoolManager;
use argentum_user_business::entity::user::AuthenticatedUser;
use argentum_user_business::repository::user_repository::{
    AuthenticatedUserRepositoryTrait, ExternalUserError,
};

use argentum_standard_business::data_type::email::EmailAddress;
use std::sync::Arc;

use argentum_user_business::data_type::builder::NameBuilder;
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
                let email = match EmailAddress::try_new(u.email.clone()) {
                    Ok(e) => e,
                    Err(_) => {
                        return Err(ExternalUserError::Authenticated(Some(Box::new(
                            BrokenStoredData::Email(u.email),
                        ))))
                    }
                };

                let name_builder =
                    NameBuilder::new(u.first_name.clone()).last(Some(u.last_name.clone()));

                let name = match name_builder.try_build() {
                    Ok(n) => n,
                    Err(_) => {
                        return Err(ExternalUserError::Authenticated(Some(Box::new(
                            BrokenStoredData::UserName {
                                first: u.first_name,
                                last: Some(u.last_name),
                            },
                        ))))
                    }
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
                    let email = match EmailAddress::try_new(item.email.clone()) {
                        Ok(e) => e,
                        Err(_) => {
                            return Err(ExternalUserError::Authenticated(Some(Box::new(
                                BrokenStoredData::Email(item.email.clone()),
                            ))))
                        }
                    };

                    let name_builder = NameBuilder::new(item.first_name.clone())
                        .last(Some(item.last_name.clone()));

                    let name = match name_builder.try_build() {
                        Ok(n) => n,
                        Err(_) => {
                            return Err(ExternalUserError::Authenticated(Some(Box::new(
                                BrokenStoredData::UserName {
                                    first: item.first_name.clone(),
                                    last: Some(item.last_name.clone()),
                                },
                            ))))
                        }
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
            Some(l) => l.to_string(),
            None => "".to_string(),
        };

        let id = self.id_factory.id_to_uuid(&user.id);
        let new_user = AuthenticatedUserModel {
            id,
            created_at: user.created_at,
            first_name: user.name.first.to_string(),
            last_name: last,
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

#[derive(thiserror::Error, Debug)]
pub enum BrokenStoredData {
    #[error("Data broken. Name `{first:?}` `{last:?}` is not valid")]
    UserName { first: String, last: Option<String> },

    #[error("Data broken. Email `{0}` is not valid")]
    Email(String),
}
