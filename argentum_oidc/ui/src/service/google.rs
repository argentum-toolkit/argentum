use argentum_standard_ui::service::external_redirect;
use dioxus::prelude::*;
use url::form_urlencoded;

use crate::{GoogleClientId, OidcRedirectUrl, service::state::create_state};
const GOOGLE_SCOPE: &str = "openid email profile";

pub async fn start_google_oauth() -> Result<(), String> {
    // CSRF
    let state = create_state()?;

    let google_client_id = use_context::<GoogleClientId>().0;
    let redirect_url = use_context::<OidcRedirectUrl>().0 + "?provider=Google";

    let params = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", &google_client_id)
        .append_pair("redirect_uri", &redirect_url)
        .append_pair("scope", GOOGLE_SCOPE)
        .append_pair("state", &state)
        .append_pair("response_type", "code") // Authorization Code Flow
        .finish();

    let auth_url = format!("https://accounts.google.com/o/oauth2/v2/auth?{}", params);

    external_redirect(&auth_url)?;

    Ok(())
}
