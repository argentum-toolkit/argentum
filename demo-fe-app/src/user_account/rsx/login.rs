use crate::route::Route;
use crate::user_account::rsx::login_form::login_form::LoginWithPasswordForm;

use argentum_user_account_rest::dto::response::UserLoggedInSuccessfullyResponse;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    #[cfg(feature = "web")]
    let on_user_logged_in_successfully = {
        use crate::standard::service::redirect;
        use crate::user_account::service::ClientSideAuthenticator;

        let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        move |response: UserLoggedInSuccessfullyResponse| match response {
            UserLoggedInSuccessfullyResponse::ApplicationJson(j) => {
                authenticator().auth_user(j.0.token, j.0.user_id);

                redirect(Route::Home {});
            }
        }
    };

    #[cfg(not(feature = "web"))]
    let on_user_logged_in_successfully = move |_response: UserLoggedInSuccessfullyResponse| {};

    rsx! {
        section {
            div { class: "container",
                div { class:"pb-40",
                    h2 { class:"mb-3 text-center text-2xl font-bold text-black dark:text-white sm:text-3xl", "Sign in to your account"}

                    div {
                        class:"mt-10 sm:mx-auto sm:w-full sm:max-w-sm",

                        LoginWithPasswordForm {
                            on_user_logged_in_successfully: on_user_logged_in_successfully,
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
