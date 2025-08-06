use std::sync::Arc;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};

use crate::dto::schema::LoginWithPasswordSchema;
use crate::dto::schema::ProblemDetail;
use crate::ui::callbacks::UserLoginsWithPasswordCallbacks;
use dioxus::prelude::*;

#[derive(PartialEq)]
pub struct UserLoginsWithPasswordFormProcessor {
    callbacks: Arc<UserLoginsWithPasswordCallbacks>,
}

impl UserLoginsWithPasswordFormProcessor {
    pub fn new(callbacks: Arc<UserLoginsWithPasswordCallbacks>) -> Self {
        Self { callbacks }
    }

    fn extract_errors_from_problem_details(
        &self,
        problem: ProblemDetail,
        mut errors: Signal<Vec<String>>,
        mut violations: Signal<ViolationsDto>,
        mut inactive: Signal<bool>,
    ) {
        if let Some(body_violation) = problem.body {
            let pp = serde_json::to_string(&body_violation).unwrap();
            let v: ViolationsDto = serde_json::from_slice(pp.as_ref()).unwrap();
            if let Some(ViolationItemDto::Object(ref items)) = v.items {
                violations.set(v.clone());
            };
        };

        if 400 == problem.status {
            errors.set(vec!["Please fill the form correctly".into()]);
        } else {
            errors.set(vec![problem.detail.unwrap_or(problem.title)]);
        }

        inactive.set(false);
    }

    #[cfg(not(feature = "web"))]
    pub async fn submit(
        &self,
        auth_token: String,
        values: Signal<LoginWithPasswordSchema>,
        mut violations: Signal<ViolationsDto>,
        mut errors: Signal<Vec<String>>,
        mut disabled: Signal<bool>,
    ) {
    }

    #[cfg(feature = "web")]
    pub async fn submit(
        &self,
        auth_token: String,
        values: Signal<LoginWithPasswordSchema>,
        mut violations: Signal<ViolationsDto>,
        mut errors: Signal<Vec<String>>,
        mut disabled: Signal<bool>,
    ) {
        use std::sync::Arc;

        use crate::client::Client;
        use argentum_rest_infrastructure::data_type::AuthHeaderParams;
        use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams, HttpRequest};

        use crate::dto::operation_response_enum::UserLoginsWithPasswordOperationResponseEnum;
        use crate::dto::params::UserLoginsWithPasswordParams;
        use crate::dto::path_params::UserLoginsWithPasswordPathParams;
        use crate::dto::request::UserLoginsWithPasswordRequest;

        use crate::dto::response::Status400Response;
        use crate::dto::response::Status401Response;

        disabled.set(true);
        errors.set(vec![]);
        violations.set(Default::default());

        let client = use_context::<Signal<Arc<Client>>>();

        let req = UserLoginsWithPasswordRequest::new(
            values().into(),
            UserLoginsWithPasswordParams::new(
                UserLoginsWithPasswordPathParams::new(),
                EmptyQueryParams {},
                AuthHeaderParams::new(auth_token),
            ),
        );

        let res = client().user_logins_with_password(req).await;

        match res {
            Ok(data) => match data {
                UserLoginsWithPasswordOperationResponseEnum::Status200(r) => {
                    (self.callbacks.on_user_logged_in_successfully)(r);
                }
                UserLoginsWithPasswordOperationResponseEnum::Status400(r) => match r {
                    Status400Response::ApplicationProblemJson(j) => {
                        self.extract_errors_from_problem_details(
                            j.0.clone(),
                            errors,
                            violations,
                            disabled,
                        );
                    }
                },
                UserLoginsWithPasswordOperationResponseEnum::Status401(r) => match r {
                    Status401Response::ApplicationProblemJson(j) => {
                        self.extract_errors_from_problem_details(
                            j.0.clone(),
                            errors,
                            violations,
                            disabled,
                        );
                    }
                },
            },
            Err(e) => {
                disabled.set(false);
                errors.set(vec![
                    "Unexpected error. Please try again latter".to_string(),
                ]);

                self.callbacks.on_error.call(e);
            }
        }
    }
}
