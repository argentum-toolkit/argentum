use crate::route::Route;

use dioxus::prelude::*;

pub(crate) fn App() -> Element {
    #[cfg(feature = "web")]
    {
        use crate::user_account::service::ClientSideAuthenticator;
        use std::cell::RefCell;
        let auth = RefCell::new(ClientSideAuthenticator::new());
        use_context_provider(|| Signal::new(auth));
    }

    rsx! {
        Router::<Route> {}
    }
}
