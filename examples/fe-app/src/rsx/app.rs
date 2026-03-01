use crate::route::Route;

use argentum_standard_ui::rsx::component::dark_mode::use_dark_mode;
use dioxus::prelude::*;
use std::env;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Clone)]
pub struct GithubClientId(pub String);

#[derive(Clone)]
pub struct OidcRedirectUrl(pub String);

pub(crate) fn App() -> Element {
    //TODO: add better error handling
    let client_id = env!("AG_GITHUB_CLIENT_ID");
    let redirect_url = env!("AG_OIDC_REDIRECT_URL");

    use_context_provider(|| GithubClientId(client_id.into()));
    use_context_provider(|| OidcRedirectUrl(redirect_url.into()));

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

    argentum_oidc_ui::di::di_init("http://localhost:8082");

    use_dark_mode();

    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}
