use argentum_standard_ui::service::external_redirect;
use dioxus::prelude::*;
use url::form_urlencoded;

use crate::{GithubClientId, OidcRedirectUrl, service::state::create_state};

const SCOPE: &str = "read:user user:email";

pub async fn start_github_oauth() -> Result<(), String> {
    let state = create_state()?;

    let github_client_id = use_context::<GithubClientId>().0;
    let redirect_url = use_context::<OidcRedirectUrl>().0 + "?provider=GitHub";

    let params = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", &github_client_id)
        .append_pair("redirect_uri", &redirect_url)
        .append_pair("scope", SCOPE)
        .append_pair("state", &state)
        .append_pair("response_type", "code") // Authorization Code Flow
        .finish();

    let auth_url = format!("https://github.com/login/oauth/authorize?{}", params);

    external_redirect(&auth_url)?;

    Ok(())
}
