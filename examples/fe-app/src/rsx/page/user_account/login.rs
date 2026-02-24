use crate::{
    route::Route,
    rsx::app::{GithubClientId, OidcRedirectUrl},
};

use argentum_user_account_rest::ui::callbacks::MemberLoginsWithPasswordCallbacks;
use argentum_user_account_rest::ui::form::MemberLoginsWithPasswordForm;
use argentum_user_account_rest::ui::form_processor::MemberLoginsWithPasswordFormProcessor;
use dioxus::prelude::*;
use rand::{RngExt, distr::Alphanumeric};
use web_sys::{Location, window};

#[cfg(feature = "server")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use url::form_urlencoded;

#[component]
pub fn Login() -> Element {
    #[cfg(feature = "web")]
    let auth_token = {
        use argentum_user_account_ui::security::ClientSideAuthenticator;
        let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        authenticator
            .read()
            .get_token()
            .expect("Can't get authentication token")
    };

    #[cfg(not(feature = "web"))]
    let auth_token = "Server side rendering cant't work with authentication tokens".to_string();

    #[cfg(feature = "web")]
    let callbacks = {
        use argentum_standard_ui::service::redirect;
        use argentum_user_account_rest::dto::response::MemberLoggedInSuccessfullyResponse;
        use argentum_user_account_ui::security::ClientSideAuthenticator;

        let mut authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        let on_member_logged_in_successfully = EventHandler::new(
            move |response: MemberLoggedInSuccessfullyResponse| match response {
                MemberLoggedInSuccessfullyResponse::ApplicationJson(j) => {
                    authenticator.write().auth_user(j.0.token, j.0.user_id);

                    let _ = redirect(Route::Home {});
                }
            },
        );

        MemberLoginsWithPasswordCallbacks {
            on_member_logged_in_successfully,
            ..Default::default()
        }
    };

    #[cfg(not(feature = "web"))]
    let callbacks = MemberLoginsWithPasswordCallbacks {
        ..Default::default()
    };

    let processor = MemberLoginsWithPasswordFormProcessor::new(callbacks);

    // let keycloak_url = "http://localhost:8099";
    // let realm = "argentum-realm-dev";
    // let client_id = "argentum-fe-client-dev";
    // let redirect_uri = "http://localhost:8080/oidc/auth/callback";

    // let keycloak_github_login_url = move || {
    //     format!(
    //         "{}/realms/{}/protocol/openid-connect/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid profile email&kc_idp_hint=github",
    //         keycloak_url,
    //         realm,
    //         client_id,
    //         urlencoding::encode(redirect_uri)
    //     )
    // };

    use dioxus::prelude::*;
    // use web_sys::window;
    //
    // let on_keycloak_github_click = move |_| {
    //     if let Some(win) = window() {
    //         let location = win.location();
    //         let _ = location.set_href(&keycloak_github_login_url());
    //     }
    // };

    let mut status = use_signal(|| "Ready to sign in".to_string());

    rsx! {
        section {
            div { class: "container",
                div { class:"pb-40",
                    h2 { "Sign in to your account" }

                    MemberLoginsWithPasswordForm {
                        processor,
                        auth_token,
                    }

                    div { class: "text-center text-base mt-10",
                        a {href:"#", class:"text-sm text-primary hover:underline", "Forgot password?"}
                    }
                    div { class: "text-center text-sm mt-2",
                        "Don't you have an account?"
                        Link { class: "text-primary hover:underline pl-2", to: Route::Registration {}, "Sign Up" }
                    }

                    // div {
                    //     div { "Or login with GitHub" }
                    //     button {
                    //         onclick: on_keycloak_github_click,
                    //         class: "btn btn-github btn-primary",
                    //         "Login with GitHub"
                    //     }
                    // }

                    div {
                        div { "Or login with:" }
                        button {
                            onclick: move |_| {
                                spawn(async move {
                                    match start_github_oauth().await {
                                        Ok(_) => {
                                            status.set("Authorization flow started → check browser".to_string());
                                        }
                                        Err(e) => {
                                            status.set(format!("Error: {}", e));
                                        }
                                    }
                                });
                            },
                            class: "btn btn-github btn-primary",
                            "GitHub"
                        }
                    }
                }
            }
        }
    }
}

const SCOPE: &str = "read:user user:email";

pub async fn start_github_oauth() -> Result<(), String> {
    // CSRF
    let state: String = rand::rng()
        .sample_iter(Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    // Optional: save state in localStorage or sessionStorage
    // to verify later when callback occurs (recommended)
    if let Some(window) = window()
        && let Ok(storage) = window.session_storage()
    {
        let _ = storage
            .expect("Can't create storage object")
            .set_item("github_oauth_state", &state);
    }

    let github_client_id = use_context::<GithubClientId>().0;
    let redirect_url = use_context::<OidcRedirectUrl>().0;

    let params = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", &github_client_id)
        .append_pair("redirect_uri", &redirect_url)
        .append_pair("scope", SCOPE)
        .append_pair("state", &state)
        .append_pair("response_type", "code") // Authorization Code Flow
        .finish();

    let auth_url = format!("https://github.com/login/oauth/authorize?{}", params);

    if let Some(win) = window() {
        let location: Location = win.location();
        if let Err(e) = location.set_href(&auth_url) {
            return Err(format!("Failed to navigate: {:?}", e));
        }
    } else {
        return Err("Cannot access window object".to_string());
    }

    Ok(())
}
