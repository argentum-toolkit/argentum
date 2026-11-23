use argentum_captcha_business::data_type::Captcha;
use argentum_contact_us_rest::dto::request::MemberSendsMessageRequest;
use argentum_rest_infrastructure::data_type::error::{BadRequestError, HttpError};
use argentum_standard_business::invariant_violation::{ViolationItem, Violations};
use std::collections::BTreeMap;

pub struct DtoToMemberSendsMessageParams {}

impl DtoToMemberSendsMessageParams {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(
        &self,
        req: MemberSendsMessageRequest,
    ) -> Result<(Option<Captcha>, String, String), HttpError> {
        let mut vo = BTreeMap::new();

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

        if vo.is_empty() {
            Ok((captcha, req.body.subject, req.body.message))
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

impl Default for DtoToMemberSendsMessageParams {
    fn default() -> Self {
        Self::new()
    }
}
