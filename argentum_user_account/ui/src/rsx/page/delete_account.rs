use argentum_standard_ui::LoginUrl;
use argentum_user_account_rest::ui::callbacks::MemberDeletesAccountCallbacks;
use argentum_user_account_rest::ui::form::MemberDeletesAccountForm;
use argentum_user_account_rest::ui::form_processor::MemberDeletesAccountFormProcessor;
use dioxus::prelude::*;

#[component]
pub fn DeleteAccount() -> Element {
    let mut confirmed = use_signal(|| false);
    let login_url = use_context::<LoginUrl>().0;

    #[cfg(feature = "web")]
    let member_token = {
        use crate::security::ClientSideAuthenticator;
        let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        authenticator.read().get_user().map(|(token, _)| token)
    };

    #[cfg(not(feature = "web"))]
    let member_token = Some("Server side rendering can't submit forms".to_string());

    #[cfg(feature = "web")]
    let callbacks = {
        use crate::security::ClientSideAuthenticator;
        use argentum_standard_ui::service::external_redirect;
        use argentum_user_account_rest::dto::response::Status204Response;

        let mut authenticator = use_context::<Signal<ClientSideAuthenticator>>();
        let redirect_url = login_url.clone();

        let on_status_204 = EventHandler::new(move |_response: Status204Response| {
            authenticator.write().logout();

            let _ = external_redirect(&redirect_url);
        });

        MemberDeletesAccountCallbacks {
            on_status_204,
            ..Default::default()
        }
    };

    #[cfg(not(feature = "web"))]
    let callbacks = MemberDeletesAccountCallbacks {
        ..Default::default()
    };

    let processor = MemberDeletesAccountFormProcessor::new(callbacks, Default::default());

    rsx! {
        section {
            div { class: "container flex justify-center",
                div { class: "pb-40 w-full max-w-sm",
                    h2 { "Delete your account" }

                    match member_token {
                        None => rsx! {
                            p { class: "text-sm text-base-content/70 mt-3 mb-8",
                                "You need to sign in to delete your account."
                            }
                            div { class: "text-center text-sm mt-5",
                                Link { class: "text-primary hover:underline", to: login_url, "Sign In" }
                            }
                        },
                        Some(auth_token) => rsx! {
                            if confirmed() {
                                p { class: "text-sm text-base-content/70 mt-3 mb-8",
                                    "Enter your password to permanently delete your account."
                                }
                                div { class: "[&_form>div:last-child]:flex [&_form>div:last-child]:justify-center",
                                    MemberDeletesAccountForm {
                                        processor,
                                        auth_token,
                                        delete_account_schema: None,
                                    }
                                }
                            } else {
                                p { class: "text-sm text-error mt-3 mb-8",
                                    "Are you sure? This cannot be undone. Your account and its data will be permanently deleted."
                                }
                                button {
                                    "type": "button",
                                    class: "btn btn-error",
                                    onclick: move |_| confirmed.set(true),
                                    "I understand, continue"
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}
