use argentum_standard_business::invariant_violation::Violations;

use crate::data_type::error::{BadRequestError, HttpError};

#[derive(Default)]
pub struct BadRequestBuilder {
    body: Violations,
    path: Violations,
    query: Violations,
    headers: Violations,
}

impl BadRequestBuilder {
    pub fn new() -> Self {
        Self {
            body: Violations::new(vec![], None),
            path: Violations::new(vec![], None),
            query: Violations::new(vec![], None),
            headers: Violations::new(vec![], None),
        }
    }

    pub fn body(&mut self, violations: Violations) -> &Self {
        self.body = violations;

        self
    }

    pub fn path(&mut self, violations: Violations) -> &Self {
        self.path = violations;

        self
    }

    pub fn query(&mut self, violations: Violations) -> &Self {
        self.query = violations;

        self
    }

    pub fn header(&mut self, violations: Violations) -> &Self {
        self.headers = violations;

        self
    }

    pub fn build(&self) -> HttpError {
        HttpError::BadRequest(BadRequestError::new(
            self.body.clone(),
            self.path.clone(),
            self.query.clone(),
            self.headers.clone(),
        ))
    }
}
