use crate::db_diesel::repository::anonymous_binding_repository::AnonymousBindingRepository;
use crate::db_diesel::repository::anonymous_user_repository::AnonymousUserRepository;
use crate::db_diesel::repository::authenticated_user_repository::AuthenticatedUserRepository;
use crate::db_diesel::repository::session_repository::SessionRepository;
use argentum_standard_infrastructure::data_type::unique_id::UniqueIdFactory;
use argentum_standard_infrastructure::db_diesel::connection::pg::ConnectionPoolManager;
use argentum_user_business::di::{UserBusinessDiC, UserBusinessDiCBuilder};
use std::sync::Arc;

pub struct UserInfrastructureDiC {
    pub business_dic: UserBusinessDiC,
}

#[derive(Default)]
pub struct UserDiCBuilder {
    pub business_dic_builder: UserBusinessDiCBuilder,
}

impl UserDiCBuilder {
    pub fn new() -> Self {
        Self {
            business_dic_builder: UserBusinessDiCBuilder::default(),
        }
    }

    pub fn defalt_services(
        &mut self,
        connection: Arc<ConnectionPoolManager>,
        id_factory: Arc<UniqueIdFactory>,
    ) -> &mut Self {
        self.business_dic_builder
            .anonymous_binding_repository(Arc::new(AnonymousBindingRepository::new(
                connection.clone(),
                id_factory.clone(),
            )))
            .anonymous_user_repository(Arc::new(AnonymousUserRepository::new(
                connection.clone(),
                id_factory.clone(),
            )))
            .authenticated_user_repository(Arc::new(AuthenticatedUserRepository::new(
                connection.clone(),
                id_factory.clone(),
            )))
            .session_repository(Arc::new(SessionRepository::new(connection, id_factory)));

        self
    }

    pub fn build(&self) -> UserInfrastructureDiC {
        UserInfrastructureDiC {
            business_dic: self.business_dic_builder.build(),
        }
    }
}
