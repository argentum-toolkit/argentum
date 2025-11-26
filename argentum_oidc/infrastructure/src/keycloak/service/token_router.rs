use argentum_user_business::service::TokenRouterTrait;

pub struct TokenRouter {}

impl TokenRouter {
    pub fn new() -> Self {
        Self {}
    }
}

impl TokenRouterTrait for TokenRouter {
    fn is_guest(&self, token: &str) -> bool {
        jsonwebtoken::decode_header(token).is_err()
    }
}
