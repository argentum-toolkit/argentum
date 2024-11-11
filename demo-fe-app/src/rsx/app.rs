use crate::{
    route::Route, rsx::dark_mode::use_dark_mode,
    user_account::service::use_client_side_authenticator_provider,
};

use dioxus::prelude::*;

pub(crate) fn App() -> Element {
    use_client_side_authenticator_provider();

    use_dark_mode();

    rsx! {
        Router::<Route> {}
    }
}
