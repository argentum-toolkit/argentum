use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

#[cfg(feature = "web")]
use crate::dto::schema::ProblemDetail;

use crate::dto::schema::MemberSendsMessageSchema;
use crate::ui::callbacks::MemberSendsMessageCallbacks;
use crate::ui::input::MemberSendsMessageSchemaInputProcessor;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MemberSendsMessageFormProcessor {
    callbacks: MemberSendsMessageCallbacks,

    pub input_processor: MemberSendsMessageSchemaInputProcessor,
}

impl MemberSendsMessageFormProcessor {
    pub fn new(
        callbacks: MemberSendsMessageCallbacks,
        input_processor: MemberSendsMessageSchemaInputProcessor,
    ) -> Self {
        Self {
            callbacks,
            input_processor,
        }
    }

    #[cfg(feature = "web")]
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

            if v.items.is_some() {
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
        _auth_token: String,
        _values: Signal<MemberSendsMessageSchema>,
        mut _violations: Signal<ViolationsDto>,
        mut _errors: Signal<Vec<String>>,
        mut _disabled: Signal<bool>,
    ) {
    }

    #[cfg(feature = "web")]
    pub async fn submit(
        &self,
        auth_token: String,
        values: Signal<MemberSendsMessageSchema>,
        mut violations: Signal<ViolationsDto>,
        mut errors: Signal<Vec<String>>,
        mut disabled: Signal<bool>,
    ) {
        use crate::client::Client;
        use argentum_rest_infrastructure::data_type::AuthHeaderParams;
        use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams, HttpRequest};

        use crate::dto::operation_response_enum::MemberSendsMessageOperationResponseEnum;
        use crate::dto::params::MemberSendsMessageParams;
        use crate::dto::path_params::MemberSendsMessagePathParams;
        use crate::dto::request::MemberSendsMessageRequest;

        use crate::dto::response::Status400Response;

        disabled.set(true);
        errors.set(vec![]);
        violations.set(Default::default());

        let client = use_context::<Signal<Client>>();

        let req = MemberSendsMessageRequest::new(
            values().into(),
            MemberSendsMessageParams::new(
                MemberSendsMessagePathParams::new(),
                EmptyQueryParams {},
                AuthHeaderParams::new(auth_token),
            ),
        );

        let res = client.read().member_sends_message(req).await;

        match res {
            Ok(data) => match data {
                MemberSendsMessageOperationResponseEnum::Status201(r) => {
                    (self.callbacks.on_message_created_successfully)(r);
                }
                MemberSendsMessageOperationResponseEnum::Status400(r) => match r {
                    Status400Response::ApplicationProblemJson(j) => {
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
