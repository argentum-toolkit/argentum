use crate::route::Route;
use crate::rsx::dark_mode::DarkMode;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_sdk::storage::*;

pub(crate) fn App() -> Element {
    use_context_provider(|| Signal::new(DarkMode(false)));

    let mut token =
        use_synced_storage::<LocalStorage, Option<String>>("x_auth_token".to_string(), || None);

    // use_hook(move || {
    // use_future(move || async move {
    use_effect(move || {
        if token().is_some() {
            tracing::info!("TOKEN FROM STORAGE: {}", token.read().to_owned().unwrap());
        }
        if token().is_none() {
            //TODO: api call to log in
            *token.write() = Some(String::from("SOME-SECURE-TOKEN"));
            // token.set(Some(String::from("unknown4")))
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
