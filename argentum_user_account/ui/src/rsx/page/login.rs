use argentum_standard_ui::RegistrationUrl;
use argentum_user_account_rest::ui::callbacks::MemberLoginsWithPasswordCallbacks;
use argentum_user_account_rest::ui::form::MemberLoginsWithPasswordForm;
use argentum_user_account_rest::ui::form_processor::MemberLoginsWithPasswordFormProcessor;
use dioxus::prelude::*;

use crate::AdditionalLoginBlock;

#[component]
pub fn Login() -> Element {
    #[cfg(feature = "web")]
    let auth_token = {
        use crate::security::ClientSideAuthenticator;
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
        let home_url = use_context::<HomePageUrl>();
        use crate::security::ClientSideAuthenticator;
        use argentum_standard_ui::HomePageUrl;
        use argentum_user_account_rest::dto::response::MemberLoggedInSuccessfullyResponse;

        let mut authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        let on_member_logged_in_successfully = EventHandler::new(
            move |response: MemberLoggedInSuccessfullyResponse| match response {
                MemberLoggedInSuccessfullyResponse::ApplicationJson(j) => {
                    use argentum_standard_ui::service::external_redirect;

                    authenticator.write().auth_user(j.0.token, j.0.user_id);

                    let _ = external_redirect(&home_url.0);
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

    // use web_sys::window;
    //
    // let on_keycloak_github_click = move |_| {
    //     if let Some(win) = window() {
    //         let location = win.location();
    //         let _ = location.set_href(&keycloak_github_login_url());
    //     }
    // };

    // let mut status = use_signal(|| "Ready to sign in".to_string());

    let additional = use_context::<AdditionalLoginBlock>().0;
    let registration_url = use_context::<RegistrationUrl>().0;

    rsx! {
        section {
            div { class: "container flex justify-center",
                div { class:"pb-40 w-full max-w-sm",
                    h2 { "Sign in to your account" }

                    MemberLoginsWithPasswordForm {
                        processor,
                        auth_token,
                    }

                    div { class: "text-base mt-10",
                        a {href:"#", class:"text-sm text-primary hover:underline", "Forgot password?"}
                    }
                    div { class: "text-center text-sm mt-2 mb-3",
                        "Don't you have an account?"
                        Link { class: "text-primary hover:underline pl-2", to: registration_url, "Sign Up" }
                    }

                    {additional}

                    // div {
                    //     div { "Or login with GitHub" }
                    //     button {
                    //         onclick: on_keycloak_github_click,
                    //         class: "btn btn-github btn-primary",
                    //         "Login with GitHub"
                    //     }
                    // }
                }
            }
        }
    }
}
