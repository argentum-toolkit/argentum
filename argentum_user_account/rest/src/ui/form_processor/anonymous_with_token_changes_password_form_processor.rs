use std::sync::Arc;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};

use crate::dto::schema::ChangePasswordSchema;
use crate::dto::schema::ProblemDetail;
use crate::ui::callbacks::AnonymousWithTokenChangesPasswordCallbacks;
use dioxus::prelude::*;

#[derive(PartialEq)]
pub struct AnonymousWithTokenChangesPasswordFormProcessor {
    callbacks: Arc<AnonymousWithTokenChangesPasswordCallbacks>,
}

impl AnonymousWithTokenChangesPasswordFormProcessor {
    pub fn new(callbacks: Arc<AnonymousWithTokenChangesPasswordCallbacks>) -> Self {
        Self { callbacks }
    }

    fn extract_errors_from_problem_details(
        &self,
        problem: ProblemDetail,
        mut errors: Signal<Vec<String>>,
        mut violations: Signal<ViolationsDto>,
        mut inactive: Signal<bool>,
    ) -> Result<(), String> {
        if let Some(body_violation) = problem.body {
            let pp = serde_json::to_string(&body_violation).map_err(|e| {
                let msg = format!("Can't serialize violations. Error {e}");
                errors.push(msg.clone());

                msg
            })?;

            let v: ViolationsDto = serde_json::from_slice(pp.as_ref()).map_err(|e| {
                let msg = format!("Can't parse body violations. Error {e}");
                errors.push(msg.clone());

                msg
            })?;

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

        Ok(())
    }

    #[cfg(not(feature = "web"))]
    pub async fn submit(
        &self,
        auth_token: String,
        values: Signal<ChangePasswordSchema>,
        mut violations: Signal<ViolationsDto>,
        mut errors: Signal<Vec<String>>,
        mut disabled: Signal<bool>,
    ) {
    }

    #[cfg(feature = "web")]
    pub async fn submit(
        &self,
        auth_token: String,
        values: Signal<ChangePasswordSchema>,
        mut violations: Signal<ViolationsDto>,
        mut errors: Signal<Vec<String>>,
        mut disabled: Signal<bool>,
    ) {
        use std::sync::Arc;

        use crate::client::Client;
        use argentum_rest_infrastructure::data_type::AuthHeaderParams;
        use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams, HttpRequest};

        use crate::dto::operation_response_enum::AnonymousWithTokenChangesPasswordOperationResponseEnum;
        use crate::dto::params::AnonymousWithTokenChangesPasswordParams;
        use crate::dto::path_params::AnonymousWithTokenChangesPasswordPathParams;
        use crate::dto::request::AnonymousWithTokenChangesPasswordRequest;

        use crate::dto::response::Status400Response;
        use crate::dto::response::Status401Response;

        disabled.set(true);
        errors.set(vec![]);
        violations.set(Default::default());

        let client = use_context::<Signal<Arc<Client>>>();

        let req = AnonymousWithTokenChangesPasswordRequest::new(
            values().into(),
            AnonymousWithTokenChangesPasswordParams::new(
                AnonymousWithTokenChangesPasswordPathParams::new(),
                EmptyQueryParams {},
                AuthHeaderParams::new(auth_token),
            ),
        );

        let res = client().anonymous_with_token_changes_password(req).await;

        match res {
            Ok(data) => match data {
                AnonymousWithTokenChangesPasswordOperationResponseEnum::Status200(r) => {
                    (self.callbacks.on_empty_ok)(r);
                }
                AnonymousWithTokenChangesPasswordOperationResponseEnum::Status400(r) => match r {
                    Status400Response::ApplicationProblemJson(j) => {
                        let _ = self.extract_errors_from_problem_details(
                            j.0.clone(),
                            errors,
                            violations,
                            disabled,
                        );
                    }
                },
                AnonymousWithTokenChangesPasswordOperationResponseEnum::Status401(r) => match r {
                    Status401Response::ApplicationProblemJson(j) => {
                        let _ = self.extract_errors_from_problem_details(
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
