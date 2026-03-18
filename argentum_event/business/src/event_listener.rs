use crate::Event;
use async_trait::async_trait;

#[async_trait]
pub trait EventListener: Send + Sync {
    async fn handle(&self, event: &dyn Event);
}
