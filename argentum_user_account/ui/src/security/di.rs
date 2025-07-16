use argentum_user_account_rest::client::Client;
use std::sync::Arc;

use crate::security::ClientSideAuthenticator;
use crate::security::SecurityRepository;

use dioxus::prelude::*;

pub struct DiC {
    pub auth: ClientSideAuthenticator,
    pub client: Arc<Client>,
}

impl DiC {
    pub fn new(auth: ClientSideAuthenticator, client: Arc<Client>) -> DiC {
        DiC { auth, client }
    }

    pub fn use_client_provider(&self) {
        use_context_provider(|| Signal::new(self.client.clone()));
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
    let mut auth = ClientSideAuthenticator::new(client.clone(), security_repository.clone());

    DiC::new(auth, client)
}
