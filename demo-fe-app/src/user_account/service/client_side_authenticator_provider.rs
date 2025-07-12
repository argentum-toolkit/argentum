pub fn use_client_side_authenticator_provider() {
    #[cfg(feature = "web")]
    {
        use argentum_user_account_rest::client::Client;
        use argentum_user_account_ui::Security;
        use std::sync::Arc;

        use argentum_user_account_ui::ClientSideAuthenticator;
        use argentum_user_account_ui::SecurityRepository;
        use dioxus::prelude::*;

        //TODO: configure host and port
        let client = Arc::new(Client::new(
            "http://localhost:8082".to_string(),
            "/api/v1".to_string(),
        ));

        let local_storage = Arc::new(web_sys::window().unwrap().local_storage().unwrap().unwrap());
        let security_repository: Arc<SecurityRepository> =
            Arc::new(SecurityRepository::new(local_storage));
        let mut auth = ClientSideAuthenticator::new(client, security_repository.clone());

        use_context_provider(|| Signal::new(auth.clone()));

        spawn(async move {
            if security_repository.read().is_none() {
                let mut authenticator = use_context::<Signal<ClientSideAuthenticator>>()();
                let res = authenticator.fetch_new_anonymous_token().await;

                match res.clone() {
                    Ok(token) => {
                        security_repository.save(Security::Anonymous(token));
                    }
                    _ => todo!(),
                };
            }
        });
    }
}
