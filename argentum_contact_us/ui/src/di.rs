use argentum_contact_us_rest::client::Client;

use dioxus::prelude::*;

pub fn di_init(contact_us_server_url: &str) {
    let client = Client::new(contact_us_server_url, "/api/v1");
    use_context_provider(|| Signal::new(client));
}
