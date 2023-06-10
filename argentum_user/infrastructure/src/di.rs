use crate::db_diesel::repository::anonymous_binding_repository::AnonymousBindingRepository;
use crate::db_diesel::repository::anonymous_user_repository::AnonymousUserRepository;
use crate::db_diesel::repository::authenticated_user_repository::AuthenticatedUserRepository;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db_diesel::connection::pg::ConnectionPoolManager;
use argentum_user_business::repository::anonymous_binding_repository::AnonymousBindingRepositoryTrait;
use argentum_user_business::repository::user_repository::{
    AnonymousUserRepositoryTrait, AuthenticatedUserRepositoryTrait,
};
use std::sync::Arc;

pub struct UserDiC {
    pub anonymous_binding_repository: Arc<dyn AnonymousBindingRepositoryTrait>,
    pub anonymous_user_repository: Arc<dyn AnonymousUserRepositoryTrait>,
    pub authenticated_user_repository: Arc<dyn AuthenticatedUserRepositoryTrait>,
}

impl UserDiC {
    pub fn new(connection: Arc<ConnectionPoolManager>, id_factory: Arc<UniqueIdFactory>) -> Self {
        let anonymous_user_repository = Arc::new(AnonymousUserRepository::new(
            connection.clone(),
            id_factory.clone(),
        ));

        let authenticated_user_repository = Arc::new(AuthenticatedUserRepository::new(
            connection.clone(),
            id_factory.clone(),
        ));

        let anonymous_binding_repository =
            Arc::new(AnonymousBindingRepository::new(connection, id_factory));

        Self {
            anonymous_binding_repository,
            anonymous_user_repository,
            authenticated_user_repository,
        }
    }
}
