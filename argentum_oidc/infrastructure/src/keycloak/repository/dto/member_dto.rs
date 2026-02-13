use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct MemberDto {
    #[serde(rename = "sub")]
    pub id: Uuid,

    // #[serde(rename = "name")]
    // pub created_at: DateTime<Utc>,
    #[serde(rename = "given_name")]
    pub first_name: String,

    #[serde(rename = "family_name")]
    pub last_name: Option<String>,

    pub email: String,
}
