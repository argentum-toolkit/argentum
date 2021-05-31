use argentum_standard_business::data_type::id::Id;

pub struct Session {
    pub id: Id,
    pub user_id: Id,
    pub token: String,
    //device_information???
}

impl Session {
    pub fn new(id: Id, user_id: Id, token: String) -> Self {
        Session { id, user_id, token }
    }
}
