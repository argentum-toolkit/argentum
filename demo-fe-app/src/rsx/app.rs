use crate::route::Route;
use crate::rsx::dark_mode::DarkMode;
use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use std::collections::HashMap;

pub(crate) fn App() -> Element {
    use_context_provider(|| Signal::new(DarkMode(false)));

    let mut token: Signal<Option<String>> = use_signal(|| None);

    use_future(move || async move {
        let mut local_storage_token =
            use_synced_storage::<LocalStorage, Option<String>>("x_auth_token".to_string(), || None);

        if local_storage_token().is_some() {
            token.set(local_storage_token());
        } else {
            let client = reqwest::Client::new();

            let res = client
                .post("http://localhost:8082/api/v1/user-account/anonymous-register")
                .header("Accept", "application/json")
                .body("")
                .send()
                .await
                .unwrap();

            let data = res.json::<HashMap<String, String>>().await.unwrap();

            if let Some(server_token) = data.get("token") {
                local_storage_token.set(Some(server_token.to_string()));
            }

            // tkn.set(Some(String::from("unknown4")));
            token.set(local_storage_token());
        }
    });

    rsx! {
        Router::<Route> {}
        div {
            match token.read().as_ref() {
                Some(t) => rsx!{ "Token: {t}"},
                None => rsx!{"No token"},
            }
        },
    }
}
