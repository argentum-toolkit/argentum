use crate::{
    route::Route, rsx::dark_mode::use_dark_mode,
    user_account::service::use_client_side_authenticator,
};

use dioxus::prelude::*;

pub(crate) fn App() -> Element {
    use_client_side_authenticator();

    use_dark_mode();

    rsx! {
        Router::<Route> {}
    }
}
