use crate::route::Route;

#[cfg(feature = "web")]
use crate::user_account::service::ClientSideAuthenticator;
use dioxus::prelude::*;
use std::cell::RefCell;

pub(crate) fn App() -> Element {
    #[cfg(feature = "web")]
    {
        let auth = RefCell::new(ClientSideAuthenticator::new());
        use_context_provider(|| Signal::new(auth));
    }

    rsx! {
        Router::<Route> {}
    }
}
