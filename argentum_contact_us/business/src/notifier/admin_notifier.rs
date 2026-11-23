use crate::entity::Message;
use argentum_standard_business::data_type::id::Id;
use std::{error::Error, sync::Arc};

use async_trait::async_trait;

#[async_trait]
pub trait AdminNotifierTrait: Sync + Send {
    type Id: Id;

    async fn send(&self, message: &Message<Self::Id>) -> Result<(), AdminNotifierError>;
}

#[derive(thiserror::Error, Debug)]
pub enum AdminNotifierError {
    #[error("Admin notification can't be sent")]
    External(#[source] Arc<dyn Error + Send + Sync>),
}
