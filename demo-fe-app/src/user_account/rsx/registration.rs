use crate::route::Route;
use crate::standard::rsx::ErrorBlock;
use crate::standard::rsx::LabeledInput;
use crate::standard::rsx::SubmitButton;
use crate::user_account::rsx::user_name::UserNameComponent;
use argentum_rest_infrastructure::data_type::HttpParams;
use argentum_rest_infrastructure::data_type::HttpRequest;
use argentum_rest_infrastructure::data_type::{AuthHeaderParams, EmptyQueryParams};
use argentum_standard_infrastructure::invariant_violation::ViolationItemDto;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use argentum_user_account_rest::client::Client;
use argentum_user_account_rest::dto::operation_response_enum::UserRegistersWithPasswordOperationResponseEnum;
use argentum_user_account_rest::dto::params::UserRegistersWithPasswordParams;
use argentum_user_account_rest::dto::path_params::UserRegistersWithPasswordPathParams;
use argentum_user_account_rest::dto::request::UserRegistersWithPasswordRequest;
use argentum_user_account_rest::dto::response::Status400Response::ApplicationProblemJson;
use argentum_user_account_rest::dto::response::Status409Response;
use argentum_user_account_rest::dto::response::UserRegisteredSuccessfullyResponse::ApplicationJson;
use argentum_user_account_rest::dto::schema::{RegistrationWithPasswordSchema, UserName};
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};
use std::string::ToString;
use std::vec;

#[component]
pub fn Registration() -> Element {
    // TODO: get daa from this values
    let agree = use_signal(|| true);
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut name = use_signal(|| UserName::new("".to_string(), None, None));

    let mut email_violations: Signal<Option<ViolationsDto>> = use_signal(|| None);
    let mut password_violations: Signal<Option<ViolationsDto>> = use_signal(|| None);
    let mut name_violations: Signal<Option<ViolationsDto>> = use_signal(|| None);

    let mut errors: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut success: Signal<Option<&str>> = use_signal(|| None);

    let mut submit_disabled = use_signal(|| false);

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
                                form {
                                    class:"space-y-6",
                                    action:"#",
                                    method:"POST",
                                    "novalidate": true,
                                    onsubmit: move |_event| {
                                        async move {
                                            submit_disabled.set(true);
                                            email_violations.set(None);
                                            password_violations.set(None);
                                            name_violations.set(None);
                                            errors.set(vec![]);
                                            success.set(None);

                                            let client = Client::new("http://localhost:8082".to_string(), "/api/v1".to_string());

                                            let local_storage_token =
                                                use_synced_storage::<LocalStorage, Option<String>>("x_auth_token".to_string(), || None).unwrap();

                                            let req = UserRegistersWithPasswordRequest::new(
                                                RegistrationWithPasswordSchema::new(email(), name(), password()),
                                                UserRegistersWithPasswordParams::new(
                                                    UserRegistersWithPasswordPathParams::new(),
                                                    EmptyQueryParams{},
                                                    // TODO: get from localstorage
                                                    AuthHeaderParams::new(local_storage_token),
                                                ),
                                            );

                                            let res = client.user_registers_with_password(req).await ;

                                            match res {
                                                Ok(data) => match data {
                                                    UserRegistersWithPasswordOperationResponseEnum::Status201(r) => match r {
                                                        ApplicationJson(_) => {
                                                            success.set(Some("Congratulations! Your account has been created."));
                                                        }
                                                    },
                                                    UserRegistersWithPasswordOperationResponseEnum::Status400(r) => match r {
                                                        ApplicationProblemJson(j) => {
                                                            let body_violation = match j.0.body {
                                                                Some(body_violation) => {
                                                                    let pp = serde_json::to_string(&body_violation).unwrap();
                                                                    let violations: ViolationsDto = serde_json::from_slice(pp.as_ref()).unwrap();
                                                                    Some(violations)
                                                                },
                                                                None => None,
                                                            };

                                                            if let Some(violations) = body_violation {
                                                                if let Some(ViolationItemDto::Object(items))  = violations.items {
                                                                    email_violations.set(items.get("email").cloned());
                                                                    password_violations.set(items.get("password").cloned());
                                                                    name_violations.set(items.get("name").cloned());
                                                                };
                                                            };

                                                            submit_disabled.set(false);
                                                        }

                                                    },
                                                    UserRegistersWithPasswordOperationResponseEnum::Status409(r) => match r {
                                                        Status409Response::ApplicationProblemJson(j) => {
                                                            errors.set(vec![j.0.title]);

                                                            submit_disabled.set(false);
                                                        },
                                                    }
                                                },
                                                Err(e) => {
                                                    errors.set(vec!["Unexpected error. Please try again latter".to_string()]);
                                                    error!("Cant register with error: `{:?}`", e);

                                                    submit_disabled.set(false);
                                                }
                                            }
                                        }
                                    },

                                    ErrorBlock {errors: errors()}

                                    LabeledInput {
                                        id: "email".to_string(),
                                        name: "email".to_string(),
                                        label: "Email address".to_string(),
                                        input_type: "email".to_string(),
                                        value: email,
                                        violations: email_violations(),
                                        oninput: move |event: String| email.set(event),
                                    },

                                    LabeledInput {
                                        id: "password".to_string(),
                                        name: "password".to_string(),
                                        label: "Password".to_string(),
                                        input_type: "password".to_string(),
                                        value: password,
                                        violations: password_violations(),
                                        oninput: move |event: String| password.set(event),
                                    },

                                    UserNameComponent {
                                        user_name: name(),
                                        violations: name_violations(),
                                        oninput: move |event: UserName| name.set(event)
                                    }

                                    div { class: "mb-8 flex",
                                        label {
                                            "htmlFor": "checkboxLabel",
                                            class: "flex cursor-pointer select-none text-sm font-medium text-body-color",
                                            div { class: "relative",
                                                input { "type":"checkbox", id: "checkboxLabel", value: "{agree}", class: "sr-only"}
                                                div { class: "box mr-4 mt-1 flex h-5 w-5 items-center justify-center rounded border border-body-color border-opacity-20 dark:border-white dark:border-opacity-10",
                                                    span {
                                                        class: "opacity-0",
                                                        svg {
                                                            width:"11",
                                                            height:"8",
                                                            "viewBox":"0 0 11 8",
                                                            fill:"none",
                                                            xmlns:"http://www.w3.org/2000/svg",
                                                            path {
                                                                d:"M10.0915 0.951972L10.0867 0.946075L10.0813 0.940568C9.90076 0.753564 9.61034 0.753146 9.42927 0.939309L4.16201 6.22962L1.58507 3.63469C1.40401 3.44841 1.11351 3.44879 0.932892 3.63584C0.755703 3.81933 0.755703 4.10875 0.932892 4.29224L0.932878 4.29225L0.934851 4.29424L3.58046 6.95832C3.73676 7.11955 3.94983 7.2 4.1473 7.2C4.36196 7.2 4.55963 7.11773 4.71406 6.9584L10.0468 1.60234C10.2436 1.4199 10.2421 1.1339 10.0915 0.951972ZM4.2327 6.30081L4.2317 6.2998C4.23206 6.30015 4.23237 6.30049 4.23269 6.30082L4.2327 6.30081Z",
                                                                fill:"#3056D3",
                                                                stroke:"#3056D3",
                                                                "strokeWidth":"0.4",
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            span {
                                                "By creating account means you agree to the"
                                                a {href:"#0", class:"text-primary hover:underline",
                                                    "Terms and Conditions"
                                                }
                                                ", and our"
                                                a {href:"#0", class:"text-primary hover:underline",
                                                    "Privacy Policy"
                                                }
                                            }
                                        }
                                    }

                                    div {
                                        SubmitButton {
                                            title: "Sign Up".to_string(),
                                            disabled: submit_disabled(),
                                        }
                                    }

                                }

                                p { class: "text-center text-base font-medium text-body-color py-8 dark:text-body-color-dark",
                                    "Already here?"
                                    Link { class: "text-primary hover:underline pl-2", to: Route::Login {}, "Sign In" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
