use crate::service::TokenRouterTrait;

#[derive(Default)]
pub struct TokenRouterMock {}

impl TokenRouterMock {
    pub fn new() -> Self {
        Self {}
    }
}

impl TokenRouterTrait for TokenRouterMock {
    fn is_guest(&self, token: &str) -> bool {
        token == "guest"
    }
}
