use crate::route::Route;
use crate::standard::rsx::ErrorBlock;
use crate::standard::rsx::LabeledCheckbox;
use crate::standard::rsx::LabeledInput;
use crate::standard::rsx::SubmitButton;
use crate::user_account::rsx::user_name::UserNameComponent;
use argentum_user_account_rest::dto::response::UserRegisteredSuccessfullyResponse;
use argentum_user_account_rest::dto::schema::UserName;
use argentum_user_account_rest::ui::form_data::{
    UserRegistersWithPasswordFormData, UserRegistersWithPasswordProps,
};
use dioxus::prelude::*;
use std::string::ToString;

#[cfg(not(feature = "web"))]
fn create_form_boilerplate(
    _props: UserRegistersWithPasswordProps,
) -> (
    impl FnMut(Event<FormData>),
    UserRegistersWithPasswordFormData,
) {
    let on_submit = move |_| {};

    let data = UserRegistersWithPasswordFormData::new();

    (on_submit, data)
}

#[cfg(feature = "web")]
fn create_form_boilerplate(
    props: UserRegistersWithPasswordProps,
) -> (
    impl FnMut(Event<FormData>),
    UserRegistersWithPasswordFormData,
) {
    use argentum_rest_infrastructure::data_type::{
        AuthHeaderParams, EmptyQueryParams, HttpParams, HttpRequest,
    };
    use argentum_standard_infrastructure::invariant_violation::ViolationItemDto;
    use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
    use argentum_user_account_rest::client::Client;

    use argentum_user_account_rest::dto::operation_response_enum::UserRegistersWithPasswordOperationResponseEnum;
    use argentum_user_account_rest::dto::params::UserRegistersWithPasswordParams;
    use argentum_user_account_rest::dto::path_params::UserRegistersWithPasswordPathParams;
    use argentum_user_account_rest::dto::request::UserRegistersWithPasswordRequest;
    use argentum_user_account_rest::dto::response::Status400Response;
    use argentum_user_account_rest::dto::response::Status409Response;
    use argentum_user_account_rest::dto::schema::RegistrationWithPasswordSchema;

    use dioxus_logger::tracing::error;

    use crate::user_account::service::ClientSideAuthenticator;

    let mut form_data = UserRegistersWithPasswordFormData::new();

    let on_submit = {
        move |_| {
            spawn(async move {
                form_data.disabled.set(true);
                form_data.violations.email.set(None);
                form_data.violations.password.set(None);
                form_data.violations.name.set(None);
                form_data.errors.set(vec![]);

                let client =
                    Client::new("http://localhost:8082".to_string(), "/api/v1".to_string());

                
                let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

                let local_storage_token = match authenticator().anonymous_token() {
                    Some(t) => t,
                    None => {
                        form_data
                            .errors
                            .set(vec!["Can't get authentication token".to_string()]);
                        error!("Can't get authentication token");

                        form_data.disabled.set(false);

                        return;
                    }
                };

                let req = UserRegistersWithPasswordRequest::new(
                    RegistrationWithPasswordSchema::new(
                        (form_data.values.email)(),
                        (form_data.values.name)(),
                        (form_data.values.password)(),
                        (form_data.values.terms)(),
                    ),
                    UserRegistersWithPasswordParams::new(
                        UserRegistersWithPasswordPathParams::new(),
                        EmptyQueryParams {},
                        // TODO: get from localstorage
                        AuthHeaderParams::new(local_storage_token),
                    ),
                );

                let res = client.user_registers_with_password(req).await;

                match res {
                    Ok(data) => match data {
                        UserRegistersWithPasswordOperationResponseEnum::Status201(r) => {
                            props.on_user_registered_successfully.call(r);
                        }
                        UserRegistersWithPasswordOperationResponseEnum::Status400(r) => match r {
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
                                        form_data.violations.email.set(items.get("email").cloned());
                                        form_data
                                            .violations
                                            .password
                                            .set(items.get("password").cloned());
                                        form_data.violations.name.set(items.get("name").cloned());
                                    };
                                };

                                form_data.disabled.set(false);
                            }
                        },
                        UserRegistersWithPasswordOperationResponseEnum::Status409(r) => match r {
                            Status409Response::ApplicationProblemJson(j) => {
                                form_data.errors.set(vec![j.0.title]);

                                form_data.disabled.set(false);
                            }
                        },
                    },
                    Err(e) => {
                        form_data
                            .errors
                            .set(vec!["Unexpected error. Please try again latter".to_string()]);
                        error!("Cant register with error: `{:?}`", e);

                        form_data.disabled.set(false);
                    }
                }
            });
        }
    };

    (on_submit, form_data)
}

fn UserRegistersWithPasswordForm(props: UserRegistersWithPasswordProps) -> Element {
    let (on_submit, mut form_data) = create_form_boilerplate(props);

    rsx! {
        form {
            class:"space-y-6",
            action:"#",
            method:"POST",
            "novalidate": true,
            onsubmit: on_submit,

            ErrorBlock {errors: (form_data.errors)()}

            LabeledInput {
                id: "email".to_string(),
                name: "email".to_string(),
                label: "Email address".to_string(),
                input_type: "email".to_string(),
                value: form_data.values.email,
                violations: (form_data.violations.email)(),
                oninput: move |event: String| form_data.values.email.set(event),
            },

            LabeledInput {
                id: "password".to_string(),
                name: "password".to_string(),
                label: "Password".to_string(),
                input_type: "password".to_string(),
                value: form_data.values.password,
                violations: (form_data.violations.password)(),
                oninput: move |event: String| form_data.values.password.set(event),
            },

            UserNameComponent {
                user_name: (form_data.values.name)(),
                violations: (form_data.violations.name)(),
                oninput: move |event: UserName| form_data.values.name.set(event)
            }

            LabeledCheckbox {
                id: "terms".to_string(),
                name: "terms".to_string(),
                label: "By creating account means you agree to the Terms and Conditions, and our Privacy Policy".to_string(),
                value: (form_data.values.terms)(),
                violations: (form_data.violations.terms)(),
                oninput: move |event: bool| form_data.values.terms.set(event),
            }

            div {
                SubmitButton {
                    title: "Sign Up".to_string(),
                    disabled: (form_data.disabled)(),
                }
            }
        }
    }
}

#[component]
pub fn Registration() -> Element {
    let mut success: Signal<Option<&str>> = use_signal(|| None);

    let on_user_registered_successfully = move |_response: UserRegisteredSuccessfullyResponse| {
        success.set(Some("Congratulations! Your account has been created."));
    };

    rsx! {
        section {
            div { class: "container",
                div { class:"pb-40",
                    h2 { class:"mb-3 text-center text-2xl font-bold text-black dark:text-white sm:text-3xl", "Create your account"}

                    div { class:"mt-10 sm:mx-auto sm:w-full sm:max-w-sm",
                        match success() {
                            Some(success_message) => {
                                rsx! {
                                    div {
                                        class: "mt-4 text-sm text-green-600 dark:text-green-500",
                                        "{success_message}",
                                    }
                                    div {
                                        class: "mt-4 text-sm",
                                        Link { class: "text-primary hover:underline pl-2", to: Route::Login {}, "Sign In" }
                                    }
                                }
                            },
                            None => rsx! {

                                UserRegistersWithPasswordForm {
                                    on_user_registered_successfully,
                                }

                                div { class: "text-center text-base font-medium py-8",
                                    "Already here?"
                                    Link { class: "text-primary hover:underline pl-2", to: Route::Login {}, "Sign In" }
                                }

                                div { class: "text-left text-base font-medium",
                                    "Please read our "
                                    a {href:"#", class:"text-primary hover:underline",
                                        "Terms and Conditions"
                                    }
                                    " and "
                                    a {href:"#", class:"text-primary hover:underline",
                                        "Privacy Policy"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
