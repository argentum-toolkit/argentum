use crate::route::Route;
use crate::standard::rsx::{ErrorBlock, LabeledInput, SubmitButton};

use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use dioxus::prelude::*;

struct Values {
    email: Signal<String>,
    password: Signal<String>,
}

struct RsxViolations {
    email: Signal<Option<ViolationsDto>>,
    password: Signal<Option<ViolationsDto>>,
}

impl Values {
    pub fn new() -> Self {
        Self {
            email: use_signal(|| "".to_string()),
            password: use_signal(|| "".to_string()),
        }
    }
}

impl RsxViolations {
    pub fn new() -> Self {
        Self {
            email: use_signal(|| None),
            password: use_signal(|| None),
        }
    }
}

#[cfg(not(feature = "web"))]
fn create_on_submit() -> (
    impl FnMut(Event<FormData>),
    Values,
    RsxViolations,
    Signal<Vec<String>>,
    Signal<bool>,
) {
    let values = Values::new();
    let rsx_violations = RsxViolations::new();
    let errors: Signal<Vec<String>> = use_signal(|| vec![]);
    let submit_disabled = use_signal(|| false);

    let on_submit: fn(Event<FormData>) = move |_| {};

    (on_submit, values, rsx_violations, errors, submit_disabled)
}

#[cfg(feature = "web")]
fn create_on_submit() -> (
    impl FnMut(Event<FormData>),
    Values,
    RsxViolations,
    Signal<Vec<String>>,
    Signal<bool>,
) {
    use crate::standard::service::redirect;
    use crate::user_account::service::ClientSideAuthenticator;
    use argentum_rest_infrastructure::data_type::{
        AuthHeaderParams, EmptyQueryParams, HttpParams, HttpRequest,
    };
    use argentum_standard_infrastructure::invariant_violation::ViolationItemDto;
    use argentum_user_account_rest::client::Client;
    use argentum_user_account_rest::dto::operation_response_enum::UserLoginsWithPasswordOperationResponseEnum;
    use argentum_user_account_rest::dto::params::UserLoginsWithPasswordParams;
    use argentum_user_account_rest::dto::path_params::UserLoginsWithPasswordPathParams;
    use argentum_user_account_rest::dto::request::UserLoginsWithPasswordRequest;
    use argentum_user_account_rest::dto::response::Status400Response;
    use argentum_user_account_rest::dto::response::Status401Response;
    use argentum_user_account_rest::dto::response::UserLoggedInSuccessfullyResponse;
    use argentum_user_account_rest::dto::schema::LoginWithPasswordSchema;
    use dioxus_logger::tracing::error;

    let mut values = Values::new();
    let mut rsx_violations = RsxViolations::new();
    let mut errors: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut submit_disabled = use_signal(|| false);

    let on_submit = {
        let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        move |_| {
            spawn(async move {
                // rsx_violations.clear();
                rsx_violations.email.set(None);
                rsx_violations.password.set(None);
                submit_disabled.set(true);
                errors.set(vec![]);

                let client =
                    Client::new("http://localhost:8082".to_string(), "/api/v1".to_string());

                let req = UserLoginsWithPasswordRequest::new(
                    LoginWithPasswordSchema::new((values.email)(), (values.password)()),
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
                            UserLoggedInSuccessfullyResponse::ApplicationJson(j) => {
                                authenticator().auth_user(j.0.token, j.0.user_id);

                                redirect(Route::Home {});
                            }
                        },
                        UserLoginsWithPasswordOperationResponseEnum::Status400(r) => match r {
                            Status400Response::ApplicationProblemJson(j) => {
                                let body_violation = match j.0.body {
                                    Some(body_violation) => {
                                        let pp = serde_json::to_string(&body_violation).unwrap();
                                        let violations: ViolationsDto =
                                            serde_json::from_slice(pp.as_ref()).unwrap();
                                        Some(violations)
                                    }
                                    None => None,
                                };

                                if let Some(violations) = body_violation {
                                    if let Some(ViolationItemDto::Object(items)) = violations.items
                                    {
                                        rsx_violations.email.set(items.get("email").cloned());
                                        rsx_violations.password.set(items.get("password").cloned());
                                    };
                                };

                                submit_disabled.set(false);
                            }
                        },
                        UserLoginsWithPasswordOperationResponseEnum::Status401(r) => match r {
                            Status401Response::ApplicationProblemJson(j) => {
                                errors.set(vec![j.0.detail.unwrap_or(j.0.title)]);

                                submit_disabled.set(false);
                            }
                        },
                    },
                    Err(e) => {
                        submit_disabled.set(false);
                        error!("Cant get token with error: `{:?}`", e);
                    }
                }
            });
        }
    };

    (on_submit, values, rsx_violations, errors, submit_disabled)
}

#[component]
pub fn LoginWithPasswordForm() -> Element {
    let (on_submit, mut values, violations, errors, submit_disabled) = create_on_submit();

    rsx! {
        form {
            onsubmit: on_submit,
            class:"space-y-6", action:"#", method:"POST",
            "novalidate": true,

            ErrorBlock {errors: errors()}

            LabeledInput {
                //todo: id should be longer
                id: "email".to_string(),
                name: "email".to_string(),
                label: "Email address".to_string(),
                input_type: "email".to_string(),
                value: values.email,
                violations: (violations.email)(),
                oninput: move |event: String| (values.email).set(event),
            },

            LabeledInput {
                id: "password".to_string(),
                name: "password".to_string(),
                label: "Password".to_string(),
                input_type: "password".to_string(),
                value: values.password,
                violations: (violations.password)(),
                oninput: move |event: String| (values.password).set(event),
            },

            div { class:"text-sm",
                a {href:"#", class:"text-sm font-medium text-primary hover:underline", "Forgot password?"}
            }

            div {
                SubmitButton {
                    title: "Sign In".to_string(),
                    disabled: submit_disabled(),
                }
            }
        }
    }
}

#[component]
pub fn Login() -> Element {
    rsx! {
        section {
            div { class: "container",
                div { class:"pb-40",
                    h2 { class:"mb-3 text-center text-2xl font-bold text-black dark:text-white sm:text-3xl", "Sign in to your account"}

                    div {
                        class:"mt-10 sm:mx-auto sm:w-full sm:max-w-sm",

                        LoginWithPasswordForm {}

                        div { class: "text-center text-base font-medium text-body-color py-8 dark:text-body-color-dark",
                            "Don't you have an account?"
                            Link { class: "text-primary hover:underline pl-2", to: Route::Registration {}, "Sign Up" }
                        }
                    }
                }
            }
        }
    }
}
