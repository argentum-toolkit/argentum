use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use crate::dto::response::{
    EmptyOkResponse,
    Status400Response,
    Status401Response,
};


use dioxus::prelude::*;
use crate::user_account::service::ClientSideAuthenticator;
use argentum_rest_infrastructure::data_type::{
    AuthHeaderParams, EmptyQueryParams, HttpParams, HttpRequest,
};
use argentum_standard_infrastructure::invariant_violation::ViolationItemDto;
use crate::client::Client;
use crate::dto::operation_response_enum::AnonymousRequestsRestoreTokenOperationResponseEnum;
use crate::dto::params::AnonymousRequestsRestoreTokenParams;
use crate::dto::path_params::AnonymousRequestsRestoreTokenPathParams;
use crate::dto::request::AnonymousRequestsRestoreTokenRequest;
use crate::dto::schema::LoginWithPasswordSchema;
use crate::dto::schema::ProblemDetail;

use crate::ui::form_data::AnonymousRequestsRestoreTokenFormData;

use crate::ui::form_data::AnonymousRequestsRestoreTokenProps;
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

pub fn create_anonymous_requests_restore_token_web_boilerplate(
    server_url: String,
    props: AnonymousRequestsRestoreTokenProps,
) -> (impl FnMut(Event<FormData>), AnonymousRequestsRestoreTokenFormData) {
    let mut form_data = AnonymousRequestsRestoreTokenFormData::new();

    let on_submit = {
        let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        move |_| {
            spawn(async move {
                // rsx_violations.clear();
                form_data.violations.email.set(None);
                form_data.violations.password.set(None);
                form_data.disabled.set(true);
                form_data.errors.set(vec![]);

                let client =
                    Client::new(server_url, "/api/v1".to_string());

                let req = AnonymousRequestsRestoreTokenRequest::new(
                    LoginWithPasswordSchema::new(
                        (form_data.values.email)(),
                        (form_data.values.password)(),
                    ),
                    AnonymousRequestsRestoreTokenParams::new(
                        AnonymousRequestsRestoreTokenPathParams::new(),
                        EmptyQueryParams {},
                        AuthHeaderParams::new(authenticator().anonymous_token().unwrap()),
                    ),
                );

                let res = client.user_logins_with_password(req).await;

                match res {
                    Ok(data) => match data {
                        AnonymousRequestsRestoreTokenOperationResponseEnum::Status200(r) => {
                            props.on_user_logged_in_successfully.call(r);
                        }
                        AnonymousRequestsRestoreTokenOperationResponseEnum::Status400(r) => {
                            match r.clone() {
                                Status400Response::ApplicationProblemJson(j) => {
                                    let body_violations =
                                        extract_body_violations_open_api_problem_details(j.0);

                                    if let Some(violations) = body_violations {
                                        if let Some(ViolationItemDto::Object(items)) =
                                            violations.items
                                        {
                                            form_data
                                                .violations
                                                .email
                                                .set(items.get("email").cloned());
                                            form_data
                                                .violations
                                                .password
                                                .set(items.get("password").cloned());
                                        };
                                    };

                                    form_data.disabled.set(false);
                                }
                            }

                            props.on_status_400.call(r);
                        }

                        AnonymousRequestsRestoreTokenOperationResponseEnum::Status401(r) => {
                            match r.clone() {
                                Status401Response::ApplicationProblemJson(j) => {
                                    let body_violations =
                                        extract_body_violations_open_api_problem_details(j.0);

                                    if let Some(violations) = body_violations {
                                        if let Some(ViolationItemDto::Object(items)) =
                                            violations.items
                                        {
                                            form_data
                                                .violations
                                                .email
                                                .set(items.get("email").cloned());
                                            form_data
                                                .violations
                                                .password
                                                .set(items.get("password").cloned());
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
        }
    };

    (on_submit, form_data)
}
