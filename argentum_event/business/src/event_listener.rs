use async_trait::async_trait;

use crate::Event;

#[async_trait]
pub trait EventListener: Send + Sync + 'static {
    async fn handle(&self, event: &dyn Event);
}
