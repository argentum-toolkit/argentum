use argentum_user_account_rest::client::Client;
use std::sync::Arc;

use crate::security::ClientSideAuthenticator;
use crate::security::SecurityRepository;

use dioxus::prelude::*;

pub struct DiC {
    pub auth: ClientSideAuthenticator,
}

impl DiC {
    pub fn new(auth: ClientSideAuthenticator) -> DiC {
        DiC { auth }
    }

    pub fn use_client_side_authenticator_provider(&self) {
        use_context_provider(|| Signal::new(self.auth.clone()));

        spawn(async move {
            let mut authenticator = use_context::<Signal<ClientSideAuthenticator>>()();
            authenticator.init().await;
        });
    }
}

pub fn di_factory(user_accoutn_server_url: String) -> DiC {
    let client = Arc::new(Client::new(user_accoutn_server_url, "/api/v1".to_string()));

    let local_storage = Arc::new(web_sys::window().unwrap().local_storage().unwrap().unwrap());
    let security_repository: Arc<SecurityRepository> =
        Arc::new(SecurityRepository::new(local_storage));
    let mut auth = ClientSideAuthenticator::new(client, security_repository.clone());

    DiC::new(auth)
}
