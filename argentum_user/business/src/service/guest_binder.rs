use std::sync::Arc;

use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::id::Id;

use crate::{
    entity::{guest_binding::GuestBinding, user::Guest},
    repository::{
        guest_binding_repository::GuestBindingRepositoryTrait,
        session_repository::SessionRepositoryTrait,
    },
};

pub struct GuestBinder<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    guest_binding_repository: Arc<dyn GuestBindingRepositoryTrait<Id = I>>,
    session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
    logger: Arc<L>,
}

impl<I, L> GuestBinder<I, L>
where
    I: Id,
    L: LoggerTrait,
{
    pub fn new(
        guest_binding_repository: Arc<dyn GuestBindingRepositoryTrait<Id = I>>,
        session_repository: Arc<dyn SessionRepositoryTrait<Id = I>>,
        logger: Arc<L>,
    ) -> Self {
        Self {
            guest_binding_repository,
            session_repository,
            logger,
        }
    }

    pub async fn bind_and_logout_guest(&self, guest: Option<Guest<I>>, member_id: I) {
        if let Some(guest) = guest {
            match self
                .session_repository
                .delete_users_sessions(guest.id)
                .await
            {
                Ok(_) => self.logger.info("Guest session deleted"),
                Err(_) => self.logger.warning("Guest session is not deleted"),
            };

            let binding = GuestBinding::new(member_id, guest.id);

            match self.guest_binding_repository.save(&binding).await {
                Ok(_) => self.logger.info("Guest binding saved"),
                Err(_) => self.logger.warning("Guest binding is not saved"),
            }
        }
    }
}
