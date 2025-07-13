use crate::{route::Route, rsx::dark_mode::use_dark_mode};

use dioxus::prelude::*;

pub(crate) fn App() -> Element {
    #[cfg(feature = "web")]
    {
        let ua_di =
            argentum_user_account_ui::security::di::di_factory("http://localhost:8082".into());
        ua_di.use_client_side_authenticator_provider();
    }

    use_dark_mode();

    rsx! {
        document::Stylesheet {
            href: asset!("/public/tailwind.css")
        }
        Router::<Route> {}
    }
}
