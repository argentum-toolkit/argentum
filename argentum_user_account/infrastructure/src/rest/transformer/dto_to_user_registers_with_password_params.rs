use argentum_rest_infrastructure::data_type::error::{BadRequestError, HttpError};
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::invariant_violation::{ViolationItem, Violations};
use argentum_user_account_rest::dto::request::UserRegistersWithPasswordRequest;
use argentum_user_business::data_type::Name;
use argentum_user_business::data_type::builder::NameBuilder;
use std::collections::BTreeMap;

pub struct DtoToUserRegistersWithPasswordParams {}

impl DtoToUserRegistersWithPasswordParams {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(
        &self,
        req: UserRegistersWithPasswordRequest,
    ) -> Result<(Name, EmailAddress, String, bool), HttpError> {
        let mut vo = BTreeMap::new();

        let raw_name = req.body.name.clone();
        let name_result = NameBuilder::new(raw_name.first)
            .last(raw_name.last)
            .try_build();

        let name = match name_result {
            Ok(n) => Some(n),
            Err(v) => {
                vo.insert("name".to_string(), v);
                None
            }
        };

        let email_result = EmailAddress::try_new(req.body.email);

        let email = match email_result {
            Ok(e) => Some(e),
            Err(v) => {
                vo.insert("email".to_string(), v);
                None
            }
        };

        let terms_accepted = req.body.terms;

        if vo.is_empty()
            && let Some(n) = name
            && let Some(e) = email
        {
            Ok((n, e, req.body.password, terms_accepted))
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
impl Default for DtoToUserRegistersWithPasswordParams {
    fn default() -> Self {
        Self::new()
    }
}
