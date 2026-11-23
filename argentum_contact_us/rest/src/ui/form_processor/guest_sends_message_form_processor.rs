use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

#[cfg(feature = "web")]
use crate::dto::schema::ProblemDetail;

use crate::dto::schema::GuestSendsMessageSchema;
use crate::ui::callbacks::GuestSendsMessageCallbacks;
use crate::ui::input::GuestSendsMessageSchemaInputProcessor;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct GuestSendsMessageFormProcessor {
    callbacks: GuestSendsMessageCallbacks,

    pub input_processor: GuestSendsMessageSchemaInputProcessor,
}

impl GuestSendsMessageFormProcessor {
    pub fn new(
        callbacks: GuestSendsMessageCallbacks,
        input_processor: GuestSendsMessageSchemaInputProcessor,
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
        _values: Signal<GuestSendsMessageSchema>,
        mut _violations: Signal<ViolationsDto>,
        mut _errors: Signal<Vec<String>>,
        mut _disabled: Signal<bool>,
    ) {
    }

    #[cfg(feature = "web")]
    pub async fn submit(
        &self,
        values: Signal<GuestSendsMessageSchema>,
        mut violations: Signal<ViolationsDto>,
        mut errors: Signal<Vec<String>>,
        mut disabled: Signal<bool>,
    ) {
        use crate::client::Client;
        use argentum_rest_infrastructure::data_type::EmptyHeaderParams;
        use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams, HttpRequest};

        use crate::dto::operation_response_enum::GuestSendsMessageOperationResponseEnum;
        use crate::dto::params::GuestSendsMessageParams;
        use crate::dto::path_params::GuestSendsMessagePathParams;
        use crate::dto::request::GuestSendsMessageRequest;

        use crate::dto::response::Status400Response;

        disabled.set(true);
        errors.set(vec![]);
        violations.set(Default::default());

        let client = use_context::<Signal<Client>>();

        let req = GuestSendsMessageRequest::new(
            values().into(),
            GuestSendsMessageParams::new(
                GuestSendsMessagePathParams::new(),
                EmptyQueryParams {},
                EmptyHeaderParams {},
            ),
        );

        let res = client.read().guest_sends_message(req).await;

        match res {
            Ok(data) => match data {
                GuestSendsMessageOperationResponseEnum::Status201(r) => {
                    (self.callbacks.on_message_created_successfully)(r);
                }
                GuestSendsMessageOperationResponseEnum::Status400(r) => match r {
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
