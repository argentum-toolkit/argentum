use std::rc::Rc;

use argentum_user_rest::client::Client;

use dioxus::prelude::*;

pub fn di_init(user_server_url: &str) {
    let client = Client::new(user_server_url, "/api/v1");
    use_context_provider(|| Signal::new(client));

    //TODO: don't duplicate client
    let client = Rc::new(Client::new(user_server_url, "/api/v1"));
    use_context_provider(|| Signal::new(client));
}
