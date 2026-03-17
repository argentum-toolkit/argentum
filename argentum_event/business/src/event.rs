use std::any::Any;

use async_trait::async_trait;

#[async_trait]
pub trait Event: Send + Sync + Any {
    fn name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
}
