use argentum_user_rest::client::Client;
use std::sync::Arc;

use dioxus::prelude::*;

pub struct DiC {
    pub client: Arc<Client>,
}

impl DiC {
    pub fn new(client: Arc<Client>) -> DiC {
        DiC { client }
    }

    pub fn use_client_provider(&self) {
        use_context_provider(|| Signal::new(self.client.clone()));
    }
}

pub fn di_factory(user_server_url: String) -> DiC {
    let client = Arc::new(Client::new(user_server_url, "/api/v1".to_string()));
    DiC::new(client)
}
