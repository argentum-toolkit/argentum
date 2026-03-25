use argentum_standard_ui::LoginUrl;
use argentum_user_account_rest::dto::response::MemberRegisteredSuccessfullyResponse;
use argentum_user_account_rest::ui::callbacks::MemberRegistersWithPasswordCallbacks;
use argentum_user_account_rest::ui::form::MemberRegistersWithPasswordForm;

use argentum_user_account_rest::ui::form_processor::MemberRegistersWithPasswordFormProcessor;
use dioxus::prelude::*;
use std::string::ToString;

use crate::AdditionalLoginBlock;

#[component]
pub fn Registration() -> Element {
    let mut success: Signal<Option<&str>> = use_signal(|| None);

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

    let on_member_registered_successfully = EventHandler::new(
        move |_response: MemberRegisteredSuccessfullyResponse| {
            success.set(Some("Congratulations! Your account has been created. Please check your email to confirm your account."));
        },
    );

    let callbacks = MemberRegistersWithPasswordCallbacks {
        on_member_registered_successfully,
        ..Default::default()
    };

    let processor = MemberRegistersWithPasswordFormProcessor::new(callbacks);

    let additional = use_context::<AdditionalLoginBlock>().0;
    let login_url = use_context::<LoginUrl>().0;

    rsx! {
        section {
            div { class: "container flex justify-center",
                div { class:"pb-40 w-full max-w-sm",
                    h2 { "Create your account "}

                    match success() {
                        Some(success_message) => {
                            rsx! {
                                div {
                                    class: "text-sm text-success mt-10",
                                    "{success_message}",
                                }
                                div {
                                    class: "text-center text-sm mt-5",
                                    Link { class: "text-primary hover:underline", to: login_url, "Sign In" }
                                }
                            }
                        },
                        None => rsx! {
                            MemberRegistersWithPasswordForm {
                                processor,
                                auth_token,
                            }

                            div { class: "text-center text-sm mt-10",
                                "Already here?"
                                Link { class: "text-primary hover:underline pl-2", to: login_url, "Sign In" }
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

                            {additional}
                        }
                    }
                }
            }
        }
    }
}
