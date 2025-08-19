use std::rc::Rc;

use argentum_user_account_rest::client::Client;

use crate::security::ClientSideAuthenticator;
use crate::security::SecurityRepository;

use dioxus::prelude::*;

pub fn di_init(user_accoutn_server_url: &str) -> Result<(), String> {
    {
        let client = Client::new(user_accoutn_server_url, "/api/v1");
        use_context_provider(|| Signal::new(client));
    }

    {
        let local_storage = web_sys::window()
            .ok_or("Can't get window object")?
            .local_storage()
            .map_err(|e| {
                format!(
                    "Can't get local storage. Error: {}",
                    e.as_string().unwrap_or("UNKNOWN".into())
                )
            })?
            .ok_or("Can't get local storag. Object is empty")?;

        let security_repository = Rc::new(SecurityRepository::new(local_storage));

        //TODO: don't initialize client twice
        let client = Rc::new(Client::new(user_accoutn_server_url, "/api/v1"));
        let mut auth = ClientSideAuthenticator::new(client, security_repository);

        use_context_provider(|| Signal::new(auth));

        spawn(async move {
            let mut authenticator = use_context::<Signal<ClientSideAuthenticator>>();
            authenticator().init().await;
        });
    }

    Ok(())
}
