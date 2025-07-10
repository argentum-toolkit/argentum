use argentum_rest_infrastructure::data_type::{
    AuthHeaderParams, EmptyQueryParams, HttpParams, HttpRequest,
};
use argentum_standard_infrastructure::invariant_violation::ViolationItemDto;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use argentum_user_account_rest::client::Client;
use argentum_user_account_rest::dto::operation_response_enum::UserLoginsWithPasswordOperationResponseEnum;
use argentum_user_account_rest::dto::params::UserLoginsWithPasswordParams;
use argentum_user_account_rest::dto::path_params::UserLoginsWithPasswordPathParams;
use argentum_user_account_rest::dto::request::UserLoginsWithPasswordRequest;
use argentum_user_account_rest::dto::response::Status400Response;
use argentum_user_account_rest::dto::response::Status401Response;
use argentum_user_account_rest::dto::schema::LoginWithPasswordSchema;
use argentum_user_account_rest::dto::schema::ProblemDetail;

use argentum_user_account_rest::ui::form_data::UserLoginsWithPasswordCallbacks;
use argentum_user_account_rest::ui::form_data::UserLoginsWithPasswordFormData;

use dioxus::prelude::*;

fn extract_body_violations_open_api_problem_details(
    problem: ProblemDetail,
) -> Option<ViolationsDto> {
    match problem.body {
        Some(body_violation) => {
            let pp = serde_json::to_string(&body_violation).unwrap();
            let violations: ViolationsDto = serde_json::from_slice(pp.as_ref()).unwrap();
            Some(violations)
        }
        None => None,
    }
}

#[cfg(not(feature = "web"))]
pub fn create_form_boilerplate(
    _callbacks: UserLoginsWithPasswordCallbacks,
) -> (impl FnMut(Event<FormData>), UserLoginsWithPasswordFormData) {
    let on_submit = move |_| {};

    let data = UserLoginsWithPasswordFormData::new();

    (on_submit, data)
}

#[cfg(feature = "web")]
pub fn create_form_boilerplate(
    props: UserLoginsWithPasswordCallbacks,
) -> (impl FnMut(Event<FormData>), UserLoginsWithPasswordFormData) {
    use crate::user_account::service::ClientSideAuthenticator;

    let mut form_data = UserLoginsWithPasswordFormData::new();

    let on_submit = move |_event: Event<FormData>| {
        let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        spawn(async move {
            //todo:  violations.clear();
            // form_data.violations.email.set(None);
            // form_data.violations.password.set(None);
            form_data.disabled.set(true);
            form_data.errors.set(vec![]);

            let client = Client::new("http://localhost:8082".to_string(), "/api/v1".to_string());

            let req = UserLoginsWithPasswordRequest::new(
                LoginWithPasswordSchema::new(
                    (form_data.values.email)(),
                    (form_data.values.password)(),
                ),
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
                                let body_violations =
                                    extract_body_violations_open_api_problem_details(j.0);

                                if let Some(violations) = body_violations {
                                    if let Some(ViolationItemDto::Object(items)) = violations.items
                                    {
                                        // form_data
                                        //     .violations
                                        //     .email
                                        //     .set(items.get("email").cloned());
                                        // form_data
                                        //     .violations
                                        //     .password
                                        //     .set(items.get("password").cloned());
                                    };
                                };

                                form_data.disabled.set(false);
                            }
                        }

                        props.on_status_400.call(r);
                    }

                    UserLoginsWithPasswordOperationResponseEnum::Status401(r) => {
                        match r.clone() {
                            Status401Response::ApplicationProblemJson(j) => {
                                let body_violations =
                                    extract_body_violations_open_api_problem_details(j.0);

                                if let Some(violations) = body_violations {
                                    if let Some(ViolationItemDto::Object(items)) = violations.items
                                    {
                                        // form_data
                                        //     .violations
                                        //     .email
                                        //     .set(items.get("email").cloned());
                                        // form_data
                                        //     .violations
                                        //     .password
                                        //     .set(items.get("password").cloned());
                                    };
                                };

                                form_data.disabled.set(false);
                            }
                        }
                        props.on_status_401.call(r);
                    }
                },
                Err(e) => {
                    form_data.disabled.set(false);
                    props.on_error.call(e);
                }
            }
        });
    };

    (on_submit, form_data)
}
