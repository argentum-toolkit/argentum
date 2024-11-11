pub fn use_client_side_authenticator_provider() {
    #[cfg(feature = "web")]
    {
        use dioxus::prelude::*;
        use crate::user_account::service::ClientSideAuthenticator;

        let auth = ClientSideAuthenticator::new();
        use_context_provider(|| Signal::new(auth));
    }
}
