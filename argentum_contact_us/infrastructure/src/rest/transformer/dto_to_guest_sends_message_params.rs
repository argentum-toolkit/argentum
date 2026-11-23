use argentum_captcha_business::data_type::Captcha;
use argentum_contact_us_rest::dto::request::GuestSendsMessageRequest;
use argentum_rest_infrastructure::data_type::error::{BadRequestError, HttpError};
use argentum_standard_business::data_type::EmailAddress;
use argentum_standard_business::invariant_violation::{ViolationItem, Violations};
use std::collections::BTreeMap;

pub struct DtoToGuestSendsMessageParams {}

impl DtoToGuestSendsMessageParams {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(
        &self,
        req: GuestSendsMessageRequest,
    ) -> Result<(Option<Captcha>, String, EmailAddress, String, String), HttpError> {
        let mut vo = BTreeMap::new();

        let email_result = EmailAddress::try_new(&req.body.email);

        let email = match email_result {
            Ok(e) => Some(e),
            Err(v) => {
                vo.insert("email".to_string(), v);
                None
            }
        };

        let captcha = match req
            .body
            .captcha
            .as_deref()
            .map(Captcha::try_new)
            .transpose()
        {
            Ok(c) => c,
            Err(v) => {
                vo.insert("captcha".to_string(), v);
                None
            }
        };

        if vo.is_empty()
            && let Some(e) = email
        {
            Ok((
                captcha,
                req.body.name,
                e,
                req.body.subject,
                req.body.message,
            ))
        } else {
            Err(HttpError::BadRequest(BadRequestError::new(
                Violations::new(vec![], Some(ViolationItem::Object(vo))),
                Violations::new(vec![], None),
                Violations::new(vec![], None),
                Violations::new(vec![], None),
            )))
        }
    }
}

impl Default for DtoToGuestSendsMessageParams {
    fn default() -> Self {
        Self::new()
    }
}
