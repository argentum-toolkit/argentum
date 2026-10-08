use argentum_user_account_rest::{
    dto::schema::ChangePasswordSchema,
    ui::{
        callbacks::GuestWithTokenChangesPasswordCallbacks, form::GuestWithTokenChangesPasswordForm,
        form_processor::GuestWithTokenChangesPasswordFormProcessor,
    },
};
use dioxus::prelude::*;

#[component]
pub fn ResetPassword(token: String) -> Element {
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
    let callbacks = GuestWithTokenChangesPasswordCallbacks {
        on_empty_ok,
        ..Default::default()
    };
    let processor = GuestWithTokenChangesPasswordFormProcessor::new(callbacks, Default::default());
    let schema = ChangePasswordSchema::new(token, String::new());

    rsx! {
        section {
            div { class: "container flex justify-center",
                div { class: "pb-40 w-full max-w-sm",
                    h2 { "Reset your password" }
                    p { class: "text-sm text-base-content/70 mt-3 mb-8",
                        "Choose a new password for your account. Use a password you don't use anywhere else."
                    }

                    if success() {
                        div { class: "text-sm text-success mt-10",
                            "Your password has been changed successfully."
                        }
                        div { class: "text-center text-sm mt-5",
                            a { class: "text-primary hover:underline", href: "/user-account/login", "Sign in" }
                        }
                    } else {
                        div { class: "[&_form>div:last-child]:flex [&_form>div:last-child]:justify-center",
                            GuestWithTokenChangesPasswordForm {
                                processor,
                                auth_token,
                                change_password_schema: Some(schema),
                            }
                        }

                        p { class: "text-xs text-base-content/60 mt-5",
                            "This reset link is temporary and can only be used once."
                        }
                    }
                }
            }
        }
    }
}
