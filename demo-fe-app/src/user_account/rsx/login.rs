use crate::route::Route;
use crate::standard::rsx::{ErrorBlock, LabeledInput, SubmitButton};

use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use argentum_user_account_rest::dto::response::{
    Status400Response, Status401Response, UserLoggedInSuccessfullyResponse,
};
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
struct LoginWithPasswordFormData {
    values: Values,
    violations: RsxViolations,
    errors: Signal<Vec<String>>,
    disabled: Signal<bool>,
}

#[cfg(not(feature = "web"))]
fn create_form_boilerplate(
    _props: LoginWithPasswordProps,
) -> (impl FnMut(Event<FormData>), LoginWithPasswordFormData) {
    let values = Values::new();
    let violations = RsxViolations::new();
    let errors: Signal<Vec<String>> = use_signal(|| vec![]);
    let disabled = use_signal(|| false);

    let on_submit: fn(Event<FormData>) = move |_| {};
    let data = LoginWithPasswordFormData {
        values,
        violations,
        errors,
        disabled,
    };

    (on_submit, data)
}

#[cfg(feature = "web")]
fn create_form_boilerplate(
    props: LoginWithPasswordProps,
) -> (impl FnMut(Event<FormData>), LoginWithPasswordFormData) {
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
    let mut disabled = use_signal(|| false);

    let on_submit = {
        let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        move |_| {
            spawn(async move {
                // rsx_violations.clear();
                rsx_violations.email.set(None);
                rsx_violations.password.set(None);
                disabled.set(true);
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
                        UserLoginsWithPasswordOperationResponseEnum::Status200(r) => {
                            props.on_user_logged_in_successfully.call(r);
                        }
                        UserLoginsWithPasswordOperationResponseEnum::Status400(r) => {
                            match r.clone() {
                                Status400Response::ApplicationProblemJson(j) => {
                                    let body_violation = match j.0.body {
                                        Some(body_violation) => {
                                            let pp =
                                                serde_json::to_string(&body_violation).unwrap();
                                            let violations: ViolationsDto =
                                                serde_json::from_slice(pp.as_ref()).unwrap();
                                            Some(violations)
                                        }
                                        None => None,
                                    };

                                    if let Some(violations) = body_violation {
                                        if let Some(ViolationItemDto::Object(items)) =
                                            violations.items
                                        {
                                            rsx_violations.email.set(items.get("email").cloned());
                                            rsx_violations
                                                .password
                                                .set(items.get("password").cloned());
                                        };
                                    };

                                    disabled.set(false);
                                }
                            }

                            props.on_status_400.call(r);
                        }
                        UserLoginsWithPasswordOperationResponseEnum::Status401(r) => {
                            match r.clone() {
                                Status401Response::ApplicationProblemJson(j) => {
                                    errors.set(vec![j.0.detail.unwrap_or(j.0.title)]);

                                    let body_violation = match j.0.body {
                                        Some(body_violation) => {
                                            let pp =
                                                serde_json::to_string(&body_violation).unwrap();
                                            let violations: ViolationsDto =
                                                serde_json::from_slice(pp.as_ref()).unwrap();
                                            Some(violations)
                                        }
                                        None => None,
                                    };

                                    if let Some(violations) = body_violation {
                                        if let Some(ViolationItemDto::Object(items)) =
                                            violations.items
                                        {
                                            rsx_violations.email.set(items.get("email").cloned());
                                            rsx_violations
                                                .password
                                                .set(items.get("password").cloned());
                                        };
                                    };

                                    disabled.set(false);
                                }
                            }

                            props.on_status_401.call(r);
                        }
                    },
                    Err(e) => {
                        disabled.set(false);
                        error!("Cant get token with error: `{:?}`", e);
                    }
                }
            });
        }
    };

    let data = LoginWithPasswordFormData {
        values,
        violations: rsx_violations,
        errors,
        disabled,
    };

    (on_submit, data)
}

#[derive(Clone, PartialEq, Props)]
struct LoginWithPasswordProps {
    #[props(default = EventHandler::new(move |_response: UserLoggedInSuccessfullyResponse| {}))]
    on_user_logged_in_successfully: EventHandler<UserLoggedInSuccessfullyResponse>,
    #[props(default = EventHandler::new(move |_response: Status400Response| {}))]
    on_status_400: EventHandler<Status400Response>,
    #[props(default = EventHandler::new(move |_response: Status401Response| {}))]
    on_status_401: EventHandler<Status401Response>,
}

#[component]
fn LoginWithPasswordForm(props: LoginWithPasswordProps) -> Element {
    let (on_submit, mut form_data) = create_form_boilerplate(props);

    rsx! {
        form {
            onsubmit: on_submit,
            class:"space-y-6", action:"#", method:"POST",
            "novalidate": true,
            ErrorBlock {errors: (form_data.errors)()}

            LabeledInput {
                //todo: id should be longer
                id: "email".to_string(),
                name: "email".to_string(),
                label: "Email address".to_string(),
                input_type: "email".to_string(),
                value: (form_data.values).email,
                violations: (form_data.violations.email)(),
                oninput: move |event: String| (form_data.values.email).set(event),
            },

            LabeledInput {
                id: "password".to_string(),
                name: "password".to_string(),
                label: "Password".to_string(),
                input_type: "password".to_string(),
                value: form_data.values.password,
                violations: (form_data.violations.password)(),
                oninput: move |event: String| (form_data.values.password).set(event),
            },

            SubmitButton {
                title: "Sign In".to_string(),
                disabled: (form_data.disabled)(),
            }
        }
    }
}

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
