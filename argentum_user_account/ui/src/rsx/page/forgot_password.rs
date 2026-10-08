use argentum_user_account_rest::ui::{
    callbacks::GuestRequestsRestoreTokenCallbacks, form::GuestRequestsRestoreTokenForm,
    form_processor::GuestRequestsRestoreTokenFormProcessor,
};
use dioxus::prelude::*;

#[component]
pub fn ForgotPassword() -> Element {
    let mut success = use_signal(|| false);
    let auth_token = {
        #[cfg(feature = "web")]
        {
            use crate::security::ClientSideAuthenticator;
            let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

            authenticator
                .read()
                .get_token()
                .expect("Can't get authentication token")
        }

        #[cfg(not(feature = "web"))]
        {
            "Server side rendering can't submit forms".to_string()
        }
    };

    let on_empty_ok = EventHandler::new(move |_| {
        success.set(true);
    });
    let callbacks = GuestRequestsRestoreTokenCallbacks {
        on_empty_ok,
        ..Default::default()
    };
    let processor = GuestRequestsRestoreTokenFormProcessor::new(callbacks, Default::default());

    rsx! {
        section {
            div { class: "container flex justify-center",
                div { class: "pb-40 w-full max-w-sm",
                    h2 { "Forgot your password?" }
                    p { class: "text-sm text-base-content/70 mt-3 mb-8",
                        "Enter the email address associated with your account and we'll send you a password reset link."
                    }

                    if success() {
                        div { class: "text-sm text-success mt-10",
                            "If the email is registered, a password reset link has been sent. Please check your inbox."
                        }
                        div { class: "text-center text-sm mt-5",
                            a { class: "text-primary hover:underline", href: "/user-account/login", "Back to sign in" }
                        }
                    } else {
                        div { class: "[&_form>div:last-child]:flex [&_form>div:last-child]:justify-center",
                            GuestRequestsRestoreTokenForm {
                                processor,
                                auth_token,
                                request_restore_token_schema: None,
                            }
                        }

                        p { class: "text-xs text-base-content/60 mt-5",
                            "For your security, we won't reveal whether an account exists for this email address."
                        }

                        div { class: "text-center text-sm mt-5",
                            a { class: "text-primary hover:underline", href: "/user-account/login", "Back to sign in" }
                        }
                    }
                }
            }
        }
    }
}
