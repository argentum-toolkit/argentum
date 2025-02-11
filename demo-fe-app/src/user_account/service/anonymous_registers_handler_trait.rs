use async_trait::async_trait;

#[async_trait]
pub trait AnonymousRegistersHandlerTrait {
    fn execute(&self) -> Result<String, String>;
}
