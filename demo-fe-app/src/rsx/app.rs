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

    #[cfg(feature = "web")]
    {
        use crate::rsx::dark_mode::DarkMode;
        use dioxus_sdk::storage::{use_synced_storage, LocalStorage};
        use std::cell::RefCell;

        let is_dark =
            use_synced_storage::<LocalStorage, bool>("dark_mode_enabled".to_string(), || false);
        use_context_provider(|| Signal::new(DarkMode(is_dark())));
    }

    rsx! {
        Router::<Route> {}
    }
}
