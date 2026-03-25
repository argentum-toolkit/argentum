use crate::Event;
use async_trait::async_trait;

#[async_trait]
pub trait EventListener: Send + Sync {
    async fn listen(&self, event: &dyn Event);
}
