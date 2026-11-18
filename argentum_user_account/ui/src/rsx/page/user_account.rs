use crate::DeleteAccountUrl;
use argentum_standard_ui::LoginUrl;
use dioxus::prelude::*;

#[component]
pub fn UserAccount() -> Element {
    let login_url = use_context::<LoginUrl>().0;
    let delete_account_url = use_context::<DeleteAccountUrl>().0;

    #[cfg(feature = "web")]
    let is_logged_in = {
        use crate::security::ClientSideAuthenticator;
        let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        authenticator.read().get_user().is_some()
    };

    #[cfg(not(feature = "web"))]
    let is_logged_in = true;

    rsx! {
        section { class: "flex flex-col md:flex-row w-full gap-6",
            if is_logged_in {
                nav { class: "flex flex-row md:flex-col md:w-48 md:shrink-0 gap-4 text-base-content",
                    Link {
                        class: "hover:text-primary focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2",
                        to: delete_account_url,
                        "Delete account"
                    }
                }
            }
            div { class: "flex-1 min-w-0 pb-40",
                h2 { "My Account" }

                if !is_logged_in {
                    p { class: "text-sm text-base-content/70 mt-3 mb-8",
                        "You need to sign in to access your account."
                    }
                    div { class: "text-center text-sm mt-5",
                        Link { class: "text-primary hover:underline", to: login_url, "Sign In" }
                    }
                }
            }
        }
    }
}
