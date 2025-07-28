use argentum_standard_business::data_type::id::Id;
use argentum_user_account_business::token::GeneratorTrait;
use fastrand;

pub struct StringTokenGenerator {}

impl StringTokenGenerator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> StringTokenGenerator {
        StringTokenGenerator {}
    }
}

impl GeneratorTrait for StringTokenGenerator {
    fn generate(&self, _user_id: &Id) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789-_";

        let len: usize = fastrand::usize(30..=500);

        let token: String = (0..len)
            .map(|_| {
                let idx = fastrand::usize(0..=CHARSET.len() - 1);
                CHARSET[idx] as char
            })
            .collect();

        token
    }
}
