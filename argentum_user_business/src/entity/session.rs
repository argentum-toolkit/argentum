use argentum_standard_business::data_type::id::IdTrait;

pub struct Session {
    pub id: Box<dyn IdTrait>,
    pub user_id: Box<dyn IdTrait>,
    pub token: String,
    //device_information???
}

impl Session {
    pub fn new(id: Box<dyn IdTrait>, user_id: Box<dyn IdTrait>, token: String) -> Self {
        Session { id, user_id, token }
    }
}
