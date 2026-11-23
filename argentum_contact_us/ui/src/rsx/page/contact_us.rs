use argentum_contact_us_rest::ui::{
    callbacks::{GuestSendsMessageCallbacks, MemberSendsMessageCallbacks},
    form::{GuestSendsMessageForm, MemberSendsMessageForm},
    form_processor::{GuestSendsMessageFormProcessor, MemberSendsMessageFormProcessor},
};
use dioxus::prelude::*;

#[component]
pub fn ContactUs() -> Element {
    let mut success = use_signal(|| false);

    #[cfg(feature = "web")]
    let (is_member, auth_token) = {
        use argentum_user_account_ui::security::ClientSideAuthenticator;
        let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        let mut auth_state = use_signal(|| None::<(bool, String)>);

        use_effect(move || {
            let auth = authenticator.read();

            let state = match auth.get_user() {
                Some((token, _)) => Some((true, token)),
                None => auth.get_token().map(|token| (false, token)),
            };

            auth_state.set(state);
        });

        match auth_state() {
            Some((is_member, token)) => (is_member, token),
            None => (false, String::new()),
        }
    };

    #[cfg(not(feature = "web"))]
    let is_member = false;

    #[cfg(not(feature = "web"))]
    let auth_token = "Server side rendering can't submit forms".to_string();

    let on_message_created_successfully = EventHandler::new(move |_| {
        success.set(true);
    });

    let guest_processor = GuestSendsMessageFormProcessor::new(
        GuestSendsMessageCallbacks {
            on_message_created_successfully,
            ..Default::default()
        },
        Default::default(),
    );

    let member_processor = MemberSendsMessageFormProcessor::new(
        MemberSendsMessageCallbacks {
            on_message_created_successfully,
            ..Default::default()
        },
        Default::default(),
    );

    rsx! {
        section {
            div { class: "container flex justify-center",
                div { class: "pb-40 w-full max-w-sm",
                    h2 { "Contact Us" }
                    p { class: "text-sm text-base-content/70 mt-3 mb-2",
                        "Have a question or feedback? Send us a message and we will get back to you."
                    }


                    if success() {
                        div { class: "text-sm text-success mt-10",
                            "Your message has been sent. We will get back to you soon."
                        }
                    } else if is_member {
                        MemberSendsMessageForm {
                            processor: member_processor,
                            auth_token,
                            member_sends_message_schema: None,
                        }
                    } else {
                        GuestSendsMessageForm {
                            processor: guest_processor,
                            guest_sends_message_schema: None,
                        }
                    }
                }
            }
        }
    }
}
