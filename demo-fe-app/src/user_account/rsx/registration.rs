use crate::route::Route;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use argentum_standard_ui::rsx::form::{LabeledCheckbox, LabeledInput, Submit};
use argentum_standard_ui::rsx::ErrorBlock;
use argentum_user_account_rest::dto::response::{
    Status400Response, Status409Response, UserRegisteredSuccessfullyResponse,
};
use argentum_user_account_rest::dto::schema::{RegistrationWithPasswordSchema, UserName};
use argentum_user_account_rest::ui::form::UserRegistersWithPasswordForm;
use argentum_user_account_rest::ui::form_data::{
    UserRegistersWithPasswordCallbacks, UserRegistersWithPasswordFormData,
    UserRegistersWithPasswordFormProps,
};

use argentum_user_account_rest::ui::input::{RegistrationWithPasswordSchemaInput, UserNameInput};
use dioxus::prelude::*;
use dioxus_logger::tracing::event;
use std::string::ToString;

#[cfg(not(feature = "web"))]
fn create_form_boilerplate(
    _callbacks: UserRegistersWithPasswordCallbacks,
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
    callbacks: UserRegistersWithPasswordCallbacks,
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

    use argentum_user_account_ui::security::ClientSideAuthenticator;

    let mut form_data = UserRegistersWithPasswordFormData::new();

    let on_submit = move |_event: Event<FormData>| {
        spawn(async move {
            form_data.disabled.set(true);
            form_data.errors.set(vec![]);
            form_data.violations.set(Default::default());

            let client = Client::new("http://localhost:8082".to_string(), "/api/v1".to_string());

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
                (form_data.values)().into(),
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
                        (callbacks.on_user_registered_successfully)(r);
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
                                if let Some(ViolationItemDto::Object(ref items)) = violations.items
                                {
                                    form_data.violations.set(violations.clone());
                                };
                            };

                            form_data
                                .errors
                                .set(vec!["Please fill the form correctly".into()]);
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
                    form_data.disabled.set(false);
                    form_data
                        .errors
                        .set(vec!["Unexpected error. Please try again latter".to_string()]);

                    callbacks.on_error.call(e);
                }
            }
        });
    };

    (on_submit, form_data)
}

#[component]
pub fn Registration() -> Element {
    let mut success: Signal<Option<&str>> = use_signal(|| None);

    let on_user_registered_successfully =
        EventHandler::new(move |_response: UserRegisteredSuccessfullyResponse| {
            success.set(Some("Congratulations! Your account has been created."));
        });

    let (on_submit, mut form_data) = create_form_boilerplate(UserRegistersWithPasswordCallbacks {
        on_user_registered_successfully,
        ..Default::default()
    });

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
                                    on_submit,
                                    form_data,
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
