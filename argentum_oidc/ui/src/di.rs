use argentum_oidc_rest::client::Client;

use dioxus::prelude::*;

pub fn di_init(ag_keycloak_server_url: &str) {
    let client = Client::new(ag_keycloak_server_url, "/api/v1");
    use_context_provider(|| Signal::new(client));
}
