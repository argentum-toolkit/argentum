use crate::route::Route;

use argentum_standard_ui::rsx::component::dark_mode::use_dark_mode;
use dioxus::prelude::*;

pub(crate) fn App() -> Element {
    #[cfg(feature = "web")]
    {
        //TODO: use ENV
        let ua_di_result = argentum_user_account_ui::security::di::di_init("http://localhost:8082");

        if let Err(e) = ua_di_result {
            return rsx! {
                "Can't initialize FE application. Error: {e}"
            };
        }
    }

    //TODO: use ENV
    argentum_user_ui::di::di_init("http://localhost:8082");

    use_dark_mode();

    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        Router::<Route> {}
    }
}
