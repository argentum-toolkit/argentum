use argentum_standard_business::data_type::id::IdTrait;
use argentum_user_business::token::GeneratorTrait;
use rand::Rng;

pub struct StringTokenGenerator {}

impl StringTokenGenerator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> StringTokenGenerator {
        StringTokenGenerator {}
    }
}

impl GeneratorTrait for StringTokenGenerator {
    fn generate(&self, _user_id: &Box<dyn IdTrait>) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789-_";
        let mut rng = rand::thread_rng();
        let len: usize = rng.gen_range(30, 500);

        let token: String = (0..len)
            .map(|_| {
                let idx = rng.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        token
    }
}
