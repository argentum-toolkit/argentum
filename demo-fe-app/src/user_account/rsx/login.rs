use crate::route::Route;
use crate::standard::rsx::LabeledInput;

use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    #[cfg(not(feature = "web"))]
    let on_submit: fn(Event<FormData>) = move |_| {};

    #[cfg(feature = "web")]
    let on_submit = {
        use crate::user_account::service::ClientSideAuthenticator;
        use argentum_rest_infrastructure::data_type::{
            AuthHeaderParams, EmptyQueryParams, HttpParams, HttpRequest,
        };
        use argentum_user_account_rest::client::Client;
        use argentum_user_account_rest::dto::operation_response_enum::UserLoginsWithPasswordOperationResponseEnum;
        use argentum_user_account_rest::dto::params::UserLoginsWithPasswordParams;
        use argentum_user_account_rest::dto::path_params::UserLoginsWithPasswordPathParams;
        use argentum_user_account_rest::dto::request::UserLoginsWithPasswordRequest;
        use argentum_user_account_rest::dto::response::UserLoggedInSuccessfullyResponse::ApplicationJson;
        use argentum_user_account_rest::dto::schema::LoginWithPasswordSchema;
        use dioxus_logger::tracing::error;

        let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        move |_| {
            spawn(async move {
                let client =
                    Client::new("http://localhost:8082".to_string(), "/api/v1".to_string());

                let req = UserLoginsWithPasswordRequest::new(
                    LoginWithPasswordSchema::new(email(), password()),
                    UserLoginsWithPasswordParams::new(
                        UserLoginsWithPasswordPathParams::new(),
                        EmptyQueryParams {},
                        AuthHeaderParams::new(authenticator().anonymous_token().unwrap()),
                    ),
                );

                let res = client.user_logins_with_password(req).await;

                match res {
                    Ok(data) => match data {
                        UserLoginsWithPasswordOperationResponseEnum::Status200(r) => match r {
                            ApplicationJson(j) => {
                                authenticator()
                                    .auth_user(j.0.token, j.0.user_id);

                                // let nav = navigator();
                                // nav.push(Route::Home {});
                            }
                        },
                        UserLoginsWithPasswordOperationResponseEnum::Status400(_) => {
                            error!("ERR STATUS: 400");
                        }
                        UserLoginsWithPasswordOperationResponseEnum::Status401(_) => {
                            error!("ERR STATUS: 401");
                        }
                    },
                    Err(e) => {
                        error!("Cant get token with error: `{:?}`", e);
                    }
                }
            });
        }
    };

    rsx! {
        section {
            div { class: "container",
                div { class:"pb-40",
                    h2 { class:"mb-3 text-center text-2xl font-bold text-black dark:text-white sm:text-3xl", "Sign in to your account"}

                    div {

                        class:"mt-10 sm:mx-auto sm:w-full sm:max-w-sm",
                        form {
                            onsubmit: on_submit,
                            class:"space-y-6", action:"#", method:"POST",

                            LabeledInput {
                                id: "email".to_string(),
                                name: "email".to_string(),
                                label: "Email address".to_string(),
                                input_type: "email".to_string(),
                                value: email,
                                oninput: move |event: String| email.set(event),
                            },

                            LabeledInput {
                                id: "password".to_string(),
                                name: "password".to_string(),
                                label: "Password".to_string(),
                                input_type: "password".to_string(),
                                value: password,
                                oninput: move |event: String| password.set(event),
                            },
                            div { class:"text-sm",
                                    a {href:"#", class:"text-sm font-medium text-primary hover:underline", "Forgot password?"}
                                }
                            div {
                                button {
                                    "type":"submit",
                                    class:"shadow-submit dark:shadow-submit-dark flex w-full items-center justify-center rounded-sm bg-primary px-9 py-4 text-base font-medium text-white duration-300 hover:bg-primary/90",
                                    "Sign in"
                                }
                            }
                        }
                        p { class: "text-center text-base font-medium text-body-color py-8 dark:text-body-color-dark",
                            "Don't you have an account?"
                            Link { class: "text-primary hover:underline pl-2", to: Route::Registration {}, "Sign Up" }
                        }
                    }
                }
            }
        }
    }
}
