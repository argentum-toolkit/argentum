use crate::route::Route;

use argentum_standard_ui::rsx::component::dark_mode::use_dark_mode;
use dioxus::prelude::*;

pub(crate) fn App() -> Element {
    #[cfg(feature = "web")]
    {
        //TODO: use ENV
        let ua_di_result =
            argentum_user_account_ui::security::di::di_factory("http://localhost:8082".into());
        match ua_di_result {
            Ok(ua_di) => {
                ua_di.use_client_side_authenticator_provider();
                ua_di.use_client_provider();
            }
            Err(e) => {
                return rsx! {
                    "Can't initialize FE application. Error: {e}"
                };
            }
        }
    }

    //TODO: use ENV
    let u_di = argentum_user_ui::di::di_factory("http://localhost:8082".into());
    u_di.use_client_provider();

    use_dark_mode();

    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        Router::<Route> {}
    }
}
