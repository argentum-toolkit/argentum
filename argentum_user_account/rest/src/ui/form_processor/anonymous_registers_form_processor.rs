use std::sync::Arc;

use argentum_standard_infrastructure::invariant_violation::{ViolationItemDto, ViolationsDto};

use crate::dto::schema::ProblemDetail;
use crate::ui::callbacks::AnonymousRegistersCallbacks;
use dioxus::prelude::*;

#[derive(PartialEq)]
pub struct AnonymousRegistersFormProcessor {
    callbacks: Arc<AnonymousRegistersCallbacks>,
}

impl AnonymousRegistersFormProcessor {
    pub fn new(callbacks: Arc<AnonymousRegistersCallbacks>) -> Self {
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
        mut violations: Signal<ViolationsDto>,
        mut errors: Signal<Vec<String>>,
        mut disabled: Signal<bool>,
    ) {
    }

    #[cfg(feature = "web")]
    pub async fn submit(
        &self,
        mut violations: Signal<ViolationsDto>,
        mut errors: Signal<Vec<String>>,
        mut disabled: Signal<bool>,
    ) {
        use std::sync::Arc;

        use crate::client::Client;
        use argentum_rest_infrastructure::data_type::EmptyHeaderParams;
        use argentum_rest_infrastructure::data_type::EmptyRequestBody;
        use argentum_rest_infrastructure::data_type::{EmptyQueryParams, HttpParams, HttpRequest};

        use crate::dto::operation_response_enum::AnonymousRegistersOperationResponseEnum;
        use crate::dto::params::AnonymousRegistersParams;
        use crate::dto::path_params::AnonymousRegistersPathParams;
        use crate::dto::request::AnonymousRegistersRequest;

        disabled.set(true);
        errors.set(vec![]);
        violations.set(Default::default());

        let client = use_context::<Signal<Arc<Client>>>();

        let req = AnonymousRegistersRequest::new(
            EmptyRequestBody {},
            AnonymousRegistersParams::new(
                AnonymousRegistersPathParams::new(),
                EmptyQueryParams {},
                EmptyHeaderParams {},
            ),
        );

        let res = client().anonymous_registers(req).await;

        match res {
            Ok(data) => match data {
                AnonymousRegistersOperationResponseEnum::Status201(r) => {
                    (self.callbacks.on_anonymous_registered_successfully)(r);
                }
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
