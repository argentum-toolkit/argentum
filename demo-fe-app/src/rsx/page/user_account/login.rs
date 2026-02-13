use crate::route::Route;

use argentum_user_account_rest::ui::callbacks::UserLoginsWithPasswordCallbacks;
use argentum_user_account_rest::ui::form::UserLoginsWithPasswordForm;
use argentum_user_account_rest::ui::form_processor::UserLoginsWithPasswordFormProcessor;
use dioxus::prelude::*;

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
        use argentum_user_account_rest::dto::response::UserLoggedInSuccessfullyResponse;
        use argentum_user_account_ui::security::ClientSideAuthenticator;

        let mut authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        let on_user_logged_in_successfully = EventHandler::new(
            move |response: UserLoggedInSuccessfullyResponse| match response {
                UserLoggedInSuccessfullyResponse::ApplicationJson(j) => {
                    authenticator.write().auth_user(j.0.token, j.0.user_id);

                    let _ = redirect(Route::Home {});
                }
            },
        );

        UserLoginsWithPasswordCallbacks {
            on_user_logged_in_successfully,
            ..Default::default()
        }
    };

    #[cfg(not(feature = "web"))]
    let callbacks = UserLoginsWithPasswordCallbacks {
        ..Default::default()
    };

    let processor = UserLoginsWithPasswordFormProcessor::new(callbacks);

    // let keycloak_url = "http://localhost:8099";
    // let realm = "argentum-realm-dev";
    // let client_id = "argentum-fe-client-dev";
    // let redirect_uri = "http://localhost:8080/oidc/auth/callback";

    // let github_login_url = move || {
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
    // let on_github_click = move |_| {
    //     if let Some(win) = window() {
    //         let location = win.location();
    //         let _ = location.set_href(&github_login_url());
    //     }
    // };

    rsx! {
        section {
            div { class: "container",
                div { class:"pb-40",
                    h2 { "Sign in to your account" }

                    UserLoginsWithPasswordForm {
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
                    //         onclick: on_github_click,
                    //         class: "btn btn-github btn-primary",
                    //         "Login with GitHub"
                    //     }
                    // }
                }
            }
        }
    }
}
