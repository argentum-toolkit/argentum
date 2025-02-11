pub fn use_client_side_authenticator_provider() {
    #[cfg(feature = "web")]
    {
        use argentum_user_account_rest::client::Client;
        use std::sync::Arc;
        
        use super::AnonymousRegistersHandler;
        use super::ClientSideAuthenticator;
        use super::SecurityRepository;
        use dioxus::prelude::*;

        let client = Arc::new(Client::new("http://localhost:8082".to_string(), "/api/v1".to_string()));
        let handler = Arc::new(AnonymousRegistersHandler::new(client));

        let local_storage = Arc::new(web_sys::window().unwrap().local_storage().unwrap().unwrap());
        let security_repository = Arc::new(SecurityRepository::new(local_storage));
        let auth = ClientSideAuthenticator::new(handler, security_repository);

        use_context_provider(|| Signal::new(auth));
    }
}
