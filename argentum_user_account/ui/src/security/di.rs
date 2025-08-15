use argentum_user_account_rest::client::Client;
use dioxus::html::g::format;
use std::rc::Rc;
use std::sync::Arc;

use crate::security::ClientSideAuthenticator;
use crate::security::SecurityRepository;

use dioxus::prelude::*;

pub struct DiC {
    pub auth: ClientSideAuthenticator,
    pub client: Rc<Client>,
}

impl DiC {
    pub fn new(auth: ClientSideAuthenticator, client: Rc<Client>) -> DiC {
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

pub fn di_factory(user_accoutn_server_url: String) -> Result<DiC, String> {
    let client = Rc::new(Client::new(user_accoutn_server_url, "/api/v1".to_string()));

    let local_storage = Rc::new(
        web_sys::window()
            .ok_or("Can't get window object")?
            .local_storage()
            .map_err(|e| {
                format!(
                    "Can't get local storage. Error: {}",
                    e.as_string().unwrap_or("UNKNOWN".into())
                )
            })?
            .ok_or("Can't get local storag. Object is empty")?,
    );

    let security_repository: Rc<SecurityRepository> =
        Rc::new(SecurityRepository::new(local_storage));
    let mut auth = ClientSideAuthenticator::new(client.clone(), security_repository.clone());

    Ok(DiC::new(auth, client))
}
