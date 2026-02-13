pub trait TokenRouterTrait: Send + Sync {
    fn is_guest(&self, token: &str) -> bool;
}
