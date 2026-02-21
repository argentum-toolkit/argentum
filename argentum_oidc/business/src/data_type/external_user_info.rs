use argentum_standard_business::data_type::EmailAddress;
use argentum_user_business::data_type::NamePart;

pub struct ExternalUserInfo {
    pub id: String,
    pub email: EmailAddress,
    pub first_name: NamePart,
    pub last_name: Option<NamePart>,
}

impl ExternalUserInfo {
    pub fn new(
        id: &str,
        email: EmailAddress,
        first_name: NamePart,
        last_name: Option<NamePart>,
    ) -> Self {
        Self {
            id: id.into(),
            email,
            first_name,
            last_name,
        }
    }
}
