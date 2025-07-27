use crate::route::Route;
use argentum_user_account_rest::dto::response::UserRegisteredSuccessfullyResponse;
use argentum_user_account_rest::ui::callbacks::UserRegistersWithPasswordCallbacks;
use argentum_user_account_rest::ui::form::UserRegistersWithPasswordForm;

use argentum_user_account_rest::ui::form_processor::UserRegistersWithPasswordFormProcessor;
use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use std::string::ToString;
use std::sync::Arc;

#[component]
pub fn Registration() -> Element {
    let mut success: Signal<Option<&str>> = use_signal(|| None);

    #[cfg(feature = "web")]
    let auth_token = {
        use argentum_user_account_ui::security::ClientSideAuthenticator;
        let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        match authenticator().get_token() {
            Some(t) => t,
            None => {
                error!("Can't get authentication token");

                return rsx! {"Can't get authentication token"};
            }
        }
    };

    #[cfg(not(feature = "web"))]
    let auth_token = "Server side rendering cant't work with authentication tokens".to_string();

    let on_user_registered_successfully =
        EventHandler::new(move |_response: UserRegisteredSuccessfullyResponse| {
            success.set(Some("Congratulations! Your account has been created."));
        });

    let callbacks = Arc::new(UserRegistersWithPasswordCallbacks {
        on_user_registered_successfully,
        ..Default::default()
    });

    let processor = Arc::new(UserRegistersWithPasswordFormProcessor::new(
        callbacks.clone(),
    ));

    rsx! {
        section {
            div { class: "container",
                div { class:"pb-40",
                    h2 { class:"mb-3 text-center text-2xl font-bold text-black dark:text-white sm:text-3xl", "Create your account"}

                    div { class:"mt-10 sm:mx-auto sm:w-full sm:max-w-sm",
                        match success() {
                            Some(success_message) => {
                                rsx! {
                                    div {
                                        class: "mt-4 text-sm text-green-600 dark:text-green-500",
                                        "{success_message}",
                                    }
                                    div {
                                        class: "mt-4 text-sm",
                                        Link { class: "text-primary hover:underline pl-2", to: Route::Login {}, "Sign In" }
                                    }
                                }
                            },
                            None => rsx! {
                                UserRegistersWithPasswordForm {
                                    processor,
                                    auth_token,
                                }

                                div { class: "text-center text-base font-medium py-8",
                                    "Already here?"
                                    Link { class: "text-primary hover:underline pl-2", to: Route::Login {}, "Sign In" }
                                }

                                div { class: "text-left text-base font-medium",
                                    "Please read our "
                                    a {href:"#", class:"text-primary hover:underline",
                                        "Terms and Conditions"
                                    }
                                    " and "
                                    a {href:"#", class:"text-primary hover:underline",
                                        "Privacy Policy"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
