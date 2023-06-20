use argentum_rest_infrastructure::data_type::error::{BadRequestError, HttpError};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::invariant_violation::{ViolationItem, Violations};
use argentum_user_account_rest::dto::request::AnonymousRequestsRestoreTokenRequest;
use std::collections::HashMap;

pub struct DtoToAnonymousRequestsRestoreTokenParams {}

impl DtoToAnonymousRequestsRestoreTokenParams {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(
        &self,
        req: AnonymousRequestsRestoreTokenRequest,
    ) -> Result<EmailAddress, HttpError> {
        let mut vo = HashMap::new();

        let email_result = EmailAddress::try_new(req.body.email);

        let email = match email_result {
            Ok(e) => Some(e),
            Err(v) => {
                vo.insert("email".to_string(), v);
                None
            }
        };

        if vo.is_empty() {
            Ok(email.unwrap())
        } else {
            Err(HttpError::BadRequest(BadRequestError::new(
                Violations::new(vec![], Some(ViolationItem::Object(vo))),
                Violations::new(vec![], None),
                Violations::new(vec![], None),
            )))
        }
    }
}
impl Default for DtoToAnonymousRequestsRestoreTokenParams {
    fn default() -> Self {
        Self::new()
    }
}
