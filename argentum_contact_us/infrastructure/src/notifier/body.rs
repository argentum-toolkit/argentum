use argentum_contact_us_business::entity::Message;
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::entity::user::Member;

/// Builds a notification body for a message.
pub fn build_body<I>(message: &Message<I>, sender: Option<&Member<I>>) -> String
where
    I: Id,
{
    let (name, email) = match sender {
        Some(member) => (
            Some(format!(
                "{} {}",
                member.name.first,
                member
                    .name
                    .last
                    .as_ref()
                    .map(|last| last.to_string())
                    .unwrap_or_default()
            )),
            Some(member.email.as_string()),
        ),
        None => (
            message.name.clone(),
            message.email.as_ref().map(|e| e.as_string()),
        ),
    };

    format!(
        "Name: {}\nEmail: {}\n\n{}",
        name.as_deref().unwrap_or(""),
        email.unwrap_or_default(),
        message.message,
    )
}
