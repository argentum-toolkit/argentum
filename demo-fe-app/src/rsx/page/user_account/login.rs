use std::sync::Arc;

use crate::route::Route;

use argentum_user_account_rest::dto::response::UserLoggedInSuccessfullyResponse;
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

        authenticator()
            .get_token()
            .expect("Can't get authentication token")
    };

    #[cfg(not(feature = "web"))]
    let auth_token = "Server side rendering cant't work with authentication tokens".to_string();

    #[cfg(feature = "web")]
    let callbacks = {
        use argentum_standard_ui::service::redirect;
        use argentum_user_account_ui::security::ClientSideAuthenticator;

        let mut authenticator = use_context::<Signal<ClientSideAuthenticator>>()();

        let on_user_logged_in_successfully = EventHandler::new(
            move |response: UserLoggedInSuccessfullyResponse| match response {
                UserLoggedInSuccessfullyResponse::ApplicationJson(j) => {
                    authenticator.auth_user(j.0.token, j.0.user_id);

                    redirect(Route::Home {});
                }
            },
        );

        Arc::new(UserLoginsWithPasswordCallbacks {
            on_user_logged_in_successfully,
            ..Default::default()
        })
    };

    #[cfg(not(feature = "web"))]
    let callbacks = Arc::new(UserLoginsWithPasswordCallbacks {
        ..Default::default()
    });

    let processor = Arc::new(UserLoginsWithPasswordFormProcessor::new(callbacks.clone()));

    rsx! {
        section {
            div { class: "container",
                div { class:"pb-40",
                    h2 { class:"mb-3 text-center text-2xl font-bold text-black dark:text-white sm:text-3xl", "Sign in to your account"}

                    div {
                        class:"mt-10 sm:mx-auto sm:w-full sm:max-w-sm",

                        UserLoginsWithPasswordForm {
                            processor,
                            auth_token,
                        }

                        div { class:"text-center text-base font-medium text-body-color py-4 dark:text-body-color-dark",
                            a {href:"#", class:"text-sm font-medium text-primary hover:underline", "Forgot password?"}
                        }
                        div { class: "text-center text-base font-medium text-body-color py-3 dark:text-body-color-dark",
                            "Don't you have an account?"
                            Link { class: "text-primary hover:underline pl-2", to: Route::Registration {}, "Sign Up" }
                        }
                    }
                }
            }
        }
    }
}
