use argentum_standard_infrastructure::data_type::unique_id::UniqueId;
use argentum_user_account_business::token::{GuestGeneratorTrait, MemberGeneratorTrait};

pub struct JwtTokenGenerator {
    secret: String,
    ttl: i64, //seconds
}

impl JwtTokenGenerator {
    pub fn new(secret: &str, ttl: i64) -> Self {
        Self {
            secret: secret.into(),
            ttl,
        }
    }
}

impl GuestGeneratorTrait for JwtTokenGenerator {
    type Id = UniqueId;

    fn generate(&self, user_id: Self::Id) -> String {
        use jsonwebtoken::{EncodingKey, Header, encode};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        struct Claims {
            sub: String,
            exp: usize,
        }

        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(self.ttl))
            .expect("Invalid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .expect("Failed to generate JWT token")
    }
}

impl MemberGeneratorTrait for JwtTokenGenerator {
    type Id = UniqueId;

    fn generate(&self, user_id: Self::Id) -> String {
        use jsonwebtoken::{EncodingKey, Header, encode};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        struct Claims {
            sub: String,
            exp: usize,
        }

        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(self.ttl))
            .expect("Invalid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .expect("Failed to generate JWT token")
    }
}
