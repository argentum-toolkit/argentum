use crate::service::TokenRouterTrait;

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
