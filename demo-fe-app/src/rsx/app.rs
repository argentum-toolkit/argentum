use crate::{route::Route, rsx::dark_mode::DarkMode};

use dioxus::prelude::*;

pub(crate) fn App() -> Element {
    use_context_provider(|| Signal::new(DarkMode(false)));

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
