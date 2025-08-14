use crate::route::Route;
use argentum_user_account_rest::dto::response::UserRegisteredSuccessfullyResponse;
use argentum_user_account_rest::ui::callbacks::UserRegistersWithPasswordCallbacks;
use argentum_user_account_rest::ui::form::UserRegistersWithPasswordForm;

use argentum_user_account_rest::ui::form_processor::UserRegistersWithPasswordFormProcessor;
use dioxus::prelude::*;
use std::rc::Rc;
use std::string::ToString;

#[component]
pub fn Registration() -> Element {
    let mut success: Signal<Option<&str>> = use_signal(|| None);

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

    let on_user_registered_successfully =
        EventHandler::new(move |_response: UserRegisteredSuccessfullyResponse| {
            success.set(Some("Congratulations! Your account has been created."));
        });

    let callbacks = Rc::new(UserRegistersWithPasswordCallbacks {
        on_user_registered_successfully,
        ..Default::default()
    });

    let processor = Rc::new(UserRegistersWithPasswordFormProcessor::new(
        callbacks.clone(),
    ));

    rsx! {
        section {
            div { class: "container",
                div { class:"pb-40 sm:max-w-sm",
                    h2 { "Create your account "}

                    match success() {
                        Some(success_message) => {
                            rsx! {
                                div {
                                    class: "text-center text-sm text-success mt-10",
                                    "{success_message}",
                                }
                                div {
                                    class: "text-center text-sm mt-5",
                                    Link { class: "text-primary hover:underline", to: Route::Login {}, "Sign In" }
                                }
                            }
                        },
                        None => rsx! {
                            UserRegistersWithPasswordForm {
                                processor,
                                auth_token,
                            }

                            div { class: "text-center text-sm mt-10",
                                "Already here?"
                                Link { class: "text-primary hover:underline pl-2", to: Route::Login {}, "Sign In" }
                            }

                            div { class: "text-left text-sm mt-5",
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
